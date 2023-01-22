use std::fmt;

use clap::Subcommand;

use crate::prelude::*;

mod scrape;

#[derive(Debug, clap::Args)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Scrape commands
    Scrape(scrape::Args),
}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Commands::Scrape(_) => write!(f, "scrape"),
        }
    }
}

pub fn process(cmd: Commands) -> Result<()> {
    match cmd {
        Commands::Scrape(args) => scrape::process(args.command),
    }
}
