use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, path};

//============ Database Handling ============

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
    fn get_db_path() -> PathBuf {
        let home = std::env::var("HOME").expect("Could not find HOME environment variable");
        let path = Path::new(&home).join(".local/share/my-zoxide/db.json");

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Failed to create database directory");
        }

        path
    }

    pub fn load() -> Self {
        let path = Self::get_db_path();

        if path.exists() {
            if let Ok(json) = fs::read_to_string(&path) {
                if let Ok(db) = serde_json::from_str(&json) {
                    return db;
                } else {
                    eprintln!("Failed to parse database, starting with empty database");
                }
            } else {
                eprintln!("Failed to read database, starting with empty database");
            }
        }

        Database::default()
    }

    pub fn save(&self) {
        let path = Self::get_db_path();

        if let Ok(json) = serde_json::to_string_pretty(self) {
            fs::write(path, json).expect("Failed to write database");
        } else {
            eprintln!("Failed to serialize database");
        }
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

    pub fn get_matching_entries(&self, keywords: &[String]) -> Vec<(String, f64)> {
        let now = get_now_time();

        let mut matches: Vec<(String, f64)> = self
            .entries
            .iter()
            .filter(|(path, _)| {
                let path_str = path.to_lowercase();
                let path_segments: Vec<&str> = path_str.split(path::MAIN_SEPARATOR).collect();

                keywords
                    .iter()
                    .all(|kw| path_segments.contains(&kw.to_lowercase().as_str()))
            })
            .map(|(path, record)| {
                let age = now.saturating_sub(record.last_accessed) as f64;
                let score = record.score / (1.0 + age / 86400.0);
                (path.clone(), score)
            })
            .collect();

        matches.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        matches
    }
}

//============ CLI Parsing ============

#[derive(Parser)]
#[command(name = "my-zoxide")]
#[command(about = "A simple directory tracking tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Add or update a directory entry")]
    Add { path: String },

    #[command(about = "Query for directories based on keywords")]
    Query { keywords: Vec<String> },

    #[command(about = "List all tracked directories")]
    List { keywords: Vec<String> },
}

//============ Main ====================

fn get_now_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

fn get_abs_path(path: &str) -> Option<String> {
    fs::canonicalize(path)
        .ok()
        .and_then(|p| p.to_str().map(|s| s.to_string()))
}

fn main() {
    let cli = Cli::parse();

    let mut db = Database::load();

    match &cli.command {
        Commands::Add { path } => {
            let abs_path = get_abs_path(path).unwrap_or_else(|| {
                eprintln!("Invalid path: {}", path);
                std::process::exit(1);
            });

            db.add_or_update_entry(abs_path.clone());

            db.save();
        }

        Commands::Query { keywords } => {
            let matches: Vec<(String, f64)> = db.get_matching_entries(keywords);

            if let Some((best_path, _)) = matches.first() {
                println!("{}", best_path);
            } else {
                eprintln!("No matching directories found");
            }
        }

        Commands::List { keywords } => {
            let matches: Vec<(String, f64)> = db.get_matching_entries(keywords);

            for (path, score) in matches {
                println!("{} (score: {:.2})", path, score);
            }
        }
    }
}
