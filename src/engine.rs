use crate::{
    db::{Database, DirRecord},
    get_now_time,
};
use anyhow::{Result, anyhow};
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use std::path;

const RECORDS_LIMIT: usize = 1000;
const RECORDS_PRUNE_THRESHOLD: usize = 2000;
const DECAY_SUM_THRESHOLD: f64 = 10000.0;
const DECAY_RATE: f64 = 0.9;

pub struct Engine {
    db: Database,
}

impl Engine {
    pub fn new() -> Self {
        let db = Database::load().unwrap_or_default();
        Engine { db }
    }

    pub fn load_db(&mut self) -> Result<()> {
        self.db = Database::load()?;
        Ok(())
    }

    pub fn save_db(&self) -> Result<()> {
        self.db.save()?;
        Ok(())
    }

    pub fn clear_db(&mut self) -> Result<()> {
        self.db.clear()?;
        Ok(())
    }
}

impl Engine {
    fn frecency_score(record: &DirRecord) -> f64 {
        let now = crate::get_now_time();
        let time_elapsed_secs = now.saturating_sub(record.last_accessed) as f64;
        let time_elapsed_hours = time_elapsed_secs / 3600.0;

        record.score / (1.0 + time_elapsed_hours).sqrt()
    }

    pub fn check_db_entries(&mut self) {
        let mut to_remove = Vec::new();

        for (path_str, _) in &self.db.entries {
            if !std::path::Path::new(path_str).exists() {
                to_remove.push(path_str.clone());
            }
        }

        for path in to_remove {
            self.db.entries.remove(&path);
        }
    }

    fn prune_db(&mut self) {
        let mut entries: Vec<(String, DirRecord)> = self.db.entries.drain().collect();

        entries.sort_by(|a, b| {
            Self::frecency_score(&b.1)
                .partial_cmp(&Self::frecency_score(&a.1))
                .unwrap()
        });

        entries.truncate(RECORDS_LIMIT);

        self.db.entries = entries.into_iter().collect();
    }

    fn decay_scores(&mut self) {
        for record in self.db.entries.values_mut() {
            record.score *= DECAY_RATE;
        }
    }

    fn ensure_db_limits(&mut self) {
        let entries_count = self.db.entries.len();

        // len > 2 * limit -> check + prune
        if entries_count > RECORDS_PRUNE_THRESHOLD {
            self.check_db_entries();
            self.prune_db();
        }

        // every 10 add -> decay
        if entries_count % 10 == 0 {
            let total_score: f64 = self.db.entries.values().map(|r| r.score).sum();

            if total_score > DECAY_SUM_THRESHOLD {
                self.decay_scores();
            }
        }
    }

    pub fn calculate_score(
        matcher: &impl FuzzyMatcher,
        path: &str,
        record: &DirRecord,
        keyword: &str,
    ) -> Option<f64> {
        let last_segment = path.split(path::MAIN_SEPARATOR).last().unwrap_or(&"");

        let mut score: f64 = matcher.fuzzy_match(path, keyword)? as f64;
        score += matcher
            .fuzzy_match(last_segment, keyword)
            .unwrap_or_else(|| 0) as f64
            * 1.5;

        let frecency = Self::frecency_score(record);
        score *= frecency;

        Some(score)
    }

    pub fn add_entry(&mut self, path: String) -> Result<()> {
        if self.db.entries.contains_key(&path) {
            return Err(anyhow!("Entry already exists"));
        }

        let now = get_now_time();

        self.db.entries.insert(
            path,
            DirRecord {
                score: 1.0,
                last_accessed: now,
            },
        );

        self.ensure_db_limits();

        Ok(())
    }

    pub fn add_or_update_entry(&mut self, path: String) {
        let now = get_now_time();

        let entry = self.db.entries.entry(path).or_insert(DirRecord {
            score: 0.0,
            last_accessed: now,
        });

        entry.score += 1.0;
        entry.last_accessed = now;

        self.ensure_db_limits();
    }

    pub fn delete_entry(&mut self, path: &str) -> Result<()> {
        if !self.db.entries.contains_key(path) {
            return Err(anyhow!("Entry not found"));
        }

        self.db.entries.remove(path);
        Ok(())
    }

    pub fn get_matching_entries(&self, keyword: &str) -> Result<Vec<(String, f64)>> {
        let matcher = SkimMatcherV2::default();

        let mut matches: Vec<(String, f64)> = self
            .db
            .entries
            .iter()
            .filter_map(|(path, record)| {
                Self::calculate_score(&matcher, path, record, keyword)
                    .map(|score| (path.clone(), score))
            })
            .collect();

        if matches.is_empty() {
            return Err(anyhow!("No matching entries found"));
        }

        matches.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        Ok(matches)
    }
}
