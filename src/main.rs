use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use my_zoxide::{db::Database, get_abs_path};

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

    #[command(about = "Delete a directory entry")]
    Delete { path: String },

    #[command(about = "Clear all directory entries")]
    Clear,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut db = Database::load()?;

    match &cli.command {
        Commands::Add { path } => {
            let abs_path = get_abs_path(path).context("Failed to get absolute path")?;

            db.add_or_update_entry(abs_path.clone());

            db.save()?;
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

        Commands::Delete { path } => {
            let abs_path = get_abs_path(path).context("Failed to get absolute path")?;

            db.delete_entry(&abs_path);
            db.save()?;
        }

        Commands::Clear => {
            db.clear()?;
        }
    }

    Ok(())
}
