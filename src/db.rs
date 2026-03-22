use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use directories::ProjectDirs;

use crate::{engine::calculate_score, get_now_time};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DirRecord {
    pub score: f64,
    pub last_accessed: u64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Database {
    pub entries: HashMap<String, DirRecord>,
}

impl Database {
    fn get_home_dir() -> Result<PathBuf> {
        let home = ProjectDirs::from("", "", "my-zoxide")
            .context("Could not find HOME environment variable")?;
        Ok(home.data_local_dir().to_path_buf())
    }

    fn get_db_path() -> Result<PathBuf> {
        let path = Self::get_home_dir()?;

        if !path.exists() {
            fs::create_dir_all(path.clone()).context("Failed to create database directory")?;
        }

        Ok(path.join("db.json"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::get_db_path()?;

        if path.exists() {
            let json = fs::read_to_string(&path).context("Failed to read database")?;
            let db = serde_json::from_str(&json).unwrap_or_default();
            Ok(db)
        } else {
            Ok(Database::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::get_db_path()?;

        let json = serde_json::to_string_pretty(self).context("Failed to serialize database")?;
        fs::write(&path, json).context("Failed to write database")?;

        Ok(())
    }

    pub fn clear(&mut self) -> Result<()> {
        self.entries.clear();

        let home = Self::get_home_dir().expect("Failed to get home directory");

        if home.exists() {
            fs::remove_dir_all(home).context("Failed to clear database")?;
        }

        Ok(())
    }
}

impl Database {
    pub fn add_or_update_entry(&mut self, path: String) {
        let now = get_now_time();

        let entry = self.entries.entry(path).or_insert(DirRecord {
            score: 0.0,
            last_accessed: now,
        });

        entry.score += 1.0;
        entry.last_accessed = now;
    }

    pub fn delete_entry(&mut self, path: &str) {
        self.entries.remove(path);
    }

    pub fn get_matching_entries(&self, keywords: &[String]) -> Vec<(String, f64)> {
        let mut matches: Vec<(String, f64)> = self
            .entries
            .iter()
            .filter_map(|(path, record)| {
                calculate_score(path, record, keywords).map(|score| (path.clone(), score))
            })
            .collect();

        matches.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        matches
    }
}
