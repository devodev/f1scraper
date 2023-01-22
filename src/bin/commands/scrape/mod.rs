use clap::Subcommand;

use crate::prelude::*;

mod races;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Scrape Races commands
    Races(races::Args),
}

pub fn process(cmd: Commands) -> Result<()> {
    match cmd {
        Commands::Races(args) => races::process(args.command),
    }
}
