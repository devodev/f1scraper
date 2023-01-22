use clap::Subcommand;

use f1_scraper::scrape::Scraper;

use crate::prelude::*;

mod races;

pub struct ScrapeContext {
    scraper: Scraper,
}

impl ScrapeContext {
    fn new() -> Self {
        Self {
            scraper: Scraper::default(),
        }
    }
}

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
    let ctx = ScrapeContext::new();
    match cmd {
        Commands::Races(args) => races::process(ctx, args.command),
    }
}
