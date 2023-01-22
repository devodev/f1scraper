use clap::Subcommand;

use f1_scraper::scrape::Scraper;

use crate::prelude::*;

mod race;

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
    /// Scrape Race commands
    Race(race::Args),
}

pub fn process(cmd: Commands) -> Result<()> {
    let client = reqwest::blocking::Client::new();
    let ctx = ScrapeContext::new(Scraper::new(client));
    match cmd {
        Commands::Race(args) => race::process(ctx, args.command),
    }
}
