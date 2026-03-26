mod commands;
mod strings;

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
    #[command(hide = true)]
    GenerateDocs,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            println!("{}", strings::MSG_LISTING_SUBMODULES);
        }
        Commands::Is { condition } => {
            commands::is::run(condition);
        }
        Commands::GenerateDocs => {
            print!("{}", clap_markdown::help_markdown::<Cli>());
        }
    }
}
