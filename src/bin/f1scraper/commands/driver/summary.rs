use crate::commands::ScrapeContext;
use crate::prelude::*;

use f1_scraper::parse::driver::{parse_summary, DriverResultSummaryTable};
use f1_scraper::scrape::{DriverResultSummaryTarget, Scraper};

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Only scrape the page for the provided year
    year: Option<u16>,

    /// Minimim year to use when scraping driver pages
    #[arg(long, default_value_t = 1950)]
    year_min: u16,

    /// Maximum year to use when scraping driver pages
    #[arg(long, default_value_t = 2023)]
    year_max: u16,
}

pub fn process(scrape_ctx: ScrapeContext, args: Args) -> Result<()> {
    // exact year takes precedence
    if let Some(year) = args.year {
        let driver_summary = query_and_parse(&scrape_ctx.scraper, year)?;
        print(&driver_summary)?;
        return Ok(());
    }

    // range of years
    for year in args.year_min..=args.year_max {
        let driver_summary = query_and_parse(&scrape_ctx.scraper, year)?;
        print(&driver_summary)?
    }
    Ok(())
}

pub fn query_and_parse(scraper: &Scraper, year: u16) -> Result<DriverResultSummaryTable> {
    // create scrape target
    let target = DriverResultSummaryTarget::new(year)
        .with_context(|| format!("create scrape target: driver result summary {year}"))?;
    // run scrape
    let html = scraper
        .scrape(target)
        .with_context(|| format!("scrape: driver result summary {year}"))?;

    // parse html text as driver summary
    let driver_summary = parse_summary(&html, year)?;
    Ok(driver_summary)
}

fn print(driver_summary: &DriverResultSummaryTable) -> Result<()> {
    println!("{:?}", driver_summary.headers);
    for row in driver_summary.data.iter() {
        println!("{row:?}");
    }
    Ok(())
}
