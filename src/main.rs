use anyhow::Result;
use clap::{Parser, Subcommand};

use my_zoxide::{engine::Engine, get_abs_path};

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
    Query { keyword: Option<String> },

    #[command(about = "List all tracked directories")]
    List {
        keyword: Option<String>,
        #[arg(short, long, default_value_t = false, help = "Show scores in the list")]
        score: bool,
    },

    #[command(about = "Delete a directory entry")]
    Delete { path: String },

    #[command(about = "Check the database for invalid entries")]
    Check,

    #[command(about = "Clear all directory entries")]
    Clear,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut engine = Engine::new();

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
            let abs_path = get_abs_path(path)?;

            engine.add_or_update_entry(abs_path.clone());

            engine.save_db()?;
        }

        Commands::Query { keyword } => {
            let matches: Vec<(String, f64)> =
                engine.get_matching_entries(keyword.as_deref().unwrap_or(""))?;

            if let Some((best_match, _)) = matches.first() {
                println!("{}", best_match);
            }
        }

        Commands::List { keyword, score } => {
            let matches: Vec<(String, f64)> =
                engine.get_matching_entries(keyword.as_deref().unwrap_or(""))?;

            for (path, path_score) in matches {
                if *score {
                    println!("{} (score: {:.2})", path, path_score);
                } else {
                    println!("{}", path);
                }
            }
        }

        Commands::Delete { path } => {
            let abs_path = get_abs_path(path)?;

            engine.delete_entry(&abs_path)?;
            engine.save_db()?;
        }

        Commands::Clear => {
            engine.clear_db()?;
        }

        Commands::Check => {
            engine.check_db_entries();
            engine.save_db()?;
        }
    }

    Ok(())
}
