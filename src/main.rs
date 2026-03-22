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
    #[command(about = "Initialize the database for a specific shell")]
    Init { shell: String },

    #[command(about = "Add or update a directory entry")]
    Add { path: String },

    #[command(about = "Query for directories based on a keyword")]
    Query { keyword: String },

    #[command(about = "List all tracked directories")]
    List {
        keyword: String,
        #[arg(short, long, default_value_t = false, help = "Show scores in the list")]
        score: bool,
    },

    #[command(about = "Delete a directory entry")]
    Delete { path: String },

    #[command(about = "Clear all directory entries")]
    Clear,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut db = Database::load()?;

    match &cli.command {
        Commands::Init { shell } => {
            let shell = shell.to_lowercase();

            let init_script = match shell.as_str() {
                "fish" => include_str!("scripts/init.fish"),
                "bash" => include_str!("scripts/init.bash"),
                "zsh" => include_str!("scripts/init.zsh"),
                _ => {
                    eprintln!("Unsupported shell: {}", shell);
                    return Ok(());
                }
            };

            println!("{}", init_script);
        }

        Commands::Add { path } => {
            let abs_path = get_abs_path(path).context("Failed to get absolute path")?;

            db.add_or_update_entry(abs_path.clone());

            db.save()?;
        }

        Commands::Query { keyword } => {
            let matches: Vec<(String, f64)> = db.get_matching_entries(keyword);

            if let Some((best_path, _)) = matches.first() {
                println!("{}", best_path);
            } else {
                eprintln!("No matching directories found");
            }
        }

        Commands::List { keyword, score } => {
            let matches: Vec<(String, f64)> = db.get_matching_entries(keyword);

            for (path, path_score) in matches {
                if *score {
                    println!("{} (score: {:.2})", path, path_score);
                } else {
                    println!("{}", path);
                }
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
