use clap::Subcommand;

use f1_scraper::scrape::Scraper;

use crate::prelude::*;

mod races;

#[derive(Default)]
pub struct ScrapeContext {
    scraper: Scraper,
}

impl ScrapeContext {
    fn new(scraper: Scraper) -> Self {
        Self { scraper }
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
    let client = reqwest::blocking::Client::new();
    let ctx = ScrapeContext::new(Scraper::new(client));
    match cmd {
        Commands::Races(args) => races::process(ctx, args.command),
    }
}
