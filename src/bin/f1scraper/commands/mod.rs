use std::fmt;

use clap::Subcommand;

use f1scraper::scrape::Scraper;

use crate::prelude::*;

mod driver;
mod race;
mod team;

#[derive(Debug, clap::Args)]
pub struct YearFlags {
    /// Only scrape the page for the provided year
    #[arg(short, long)]
    year: Option<u16>,

    /// Minimim year to use when scraping pages
    #[arg(long, default_value_t = 1950)]
    year_min: u16,

    /// Maximum year to use when scraping pages
    #[arg(long, default_value_t = 2023)]
    year_max: u16,
}

impl YearFlags {
    fn min_max(&self) -> (u16, u16) {
        match self.year {
            Some(year) => (year, year),
            _ => (self.year_min, self.year_max),
        }
    }
}

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
        Commands::Race(args) => race::run(ctx, args.command),
        Commands::Driver(args) => driver::run(ctx, args.command),
        Commands::Team(args) => team::process(ctx, args.command),
    }
}
