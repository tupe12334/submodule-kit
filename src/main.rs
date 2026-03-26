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
    /// Check one or more conditions about submodules; exits 0 (all true) or 1 (any false)
    Is { conditions: Vec<IsCondition> },
    #[command(hide = true)]
    GenerateDocs,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            commands::is::list();
        }
        Commands::Is { conditions } => {
            commands::is::run(conditions);
        }
        Commands::GenerateDocs => {
            print!(
                "{}\n## License\n\nMIT — see [LICENSE](LICENSE)\n",
                clap_markdown::help_markdown::<Cli>()
            );
        }
    }
}
