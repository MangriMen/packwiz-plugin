use clap::command;
use clap::{Parser, Subcommand};

pub mod build;
pub mod clean;
pub mod package;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Bundle,
    Package,
    Clean,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Bundle => {
            package::bundle();
        }
        Commands::Package => package::package(),
        Commands::Clean => clean::clean(),
    }
}
