use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirRecord {
    pub score: f64,
    pub last_accessed: u64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
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

        Ok(path.join("db.bin"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::get_db_path()?;

        if path.exists() {
            let bytes = fs::read(&path).context("Failed to read database")?;
            let db = bincode::deserialize::<Self>(&bytes).unwrap_or_default();
            Ok(db)
        } else {
            Ok(Database::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::get_db_path()?;

        let bytes = bincode::serialize(self).context("Failed to serialize database")?;
        fs::write(&path, bytes).context("Failed to write database")?;

        Ok(())
    }

    pub fn clear(&mut self) -> Result<()> {
        self.entries.clear();

        let home = Self::get_home_dir().context("Failed to get database directory")?;

        if home.exists() {
            fs::remove_dir_all(home).context("Failed to clear database")?;
        }

        Ok(())
    }
}
