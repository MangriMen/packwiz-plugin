use clap::Parser;

use crate::core::{self, Cli, Commands};

pub fn run_command() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build => core::build(),
        Commands::Bundle => core::bundle(),
        Commands::Package => core::package(),
        Commands::Clean => core::clean(),
    }
}
