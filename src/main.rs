mod commands;

use clap::{Parser, Subcommand};
use commands::is::IsCondition;

#[derive(Parser)]
#[command(
    name = "submodule-kit",
    about = "A CLI toolkit for managing git submodules"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all submodules
    List,
    /// Check a condition about submodules; exits 0 (true) or 1 (false)
    Is {
        #[command(subcommand)]
        condition: IsCondition,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            println!("Listing submodules...");
        }
        Commands::Is { condition } => {
            commands::is::run(condition);
        }
    }
}
