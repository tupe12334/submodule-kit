pub mod commands;
pub mod strings;
pub mod submodule;

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
    /// Check one or more conditions about submodules; exits 0 (all true) or 1 (any false)
    Is { conditions: Vec<IsCondition> },
    #[command(hide = true)]
    GenerateDocs,
}

pub fn run() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            if let Err(e) = commands::list::run() {
                eprintln!("error: {e}");
                std::process::exit(2);
            }
        }
        Commands::Is { conditions } => match commands::is::run(conditions) {
            Ok(true) => {}
            Ok(false) => std::process::exit(1),
            Err(e) => {
                eprintln!("error: {e}");
                std::process::exit(2);
            }
        },
        Commands::GenerateDocs => {
            print!(
                "{}\n## License\n\nMIT — see [LICENSE](LICENSE)\n",
                clap_markdown::help_markdown::<Cli>()
            );
        }
    }
}
