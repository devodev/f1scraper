use std::fmt;

use clap::Subcommand;

use f1scraper::scrape::Scraper;

use crate::prelude::*;

mod driver;
mod race;
mod team;

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
pub(crate) struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Scrape races
    Race(race::Args),

    /// Scrape drivers
    Driver(driver::Args),

    /// Scrape teams
    Team(team::Args),
}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Commands::Race(_) => write!(f, "race"),
            Commands::Driver(_) => write!(f, "driver"),
            Commands::Team(_) => write!(f, "team"),
        }
    }
}

pub fn process(cmd: Commands) -> Result<()> {
    let client = reqwest::blocking::Client::new();
    let ctx = ScrapeContext::new(Scraper::new(client));
    match cmd {
        Commands::Race(args) => race::process(ctx, args.command),
        Commands::Driver(args) => driver::process(ctx, args.command),
        Commands::Team(args) => team::process(ctx, args.command),
    }
}
