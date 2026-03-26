use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "submodule-kit", about = "A CLI toolkit for managing git submodules")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all submodules
    List,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            println!("Listing submodules...");
        }
    }
}
