use crate::{
    db::{Database, DirRecord},
    get_now_time,
};
use anyhow::{Result, anyhow};
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use std::path;

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
