use f1scraper::scrape::{DriverResultSummaryTarget, Scraper};
use f1scraper::types::DriverSummary;

use crate::commands::ScrapeContext;
use crate::{prelude::*, YearFlags};

#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    year_flags: YearFlags,
}

pub fn run(scrape_ctx: ScrapeContext, args: Args) -> Result<()> {
    let (year_min, year_max) = args.year_flags.min_max();
    for year in year_min..=year_max {
        let result = query_and_parse(&scrape_ctx.scraper, year)?;
        print(&result)?
    }
    Ok(())
}

pub fn query_and_parse(scraper: &Scraper, year: u16) -> Result<DriverSummary> {
    // create scrape target
    let target = DriverResultSummaryTarget::new(year)
        .with_context(|| format!("create scrape target: driver result summary {year}"))?;
    // run scrape
    let html = scraper
        .scrape(target)
        .with_context(|| format!("scrape: driver result summary {year}"))?;
    // parse html text as driver summary
    let driver_summary = DriverSummary::parse(&html, year)?;
    Ok(driver_summary)
}

fn print(driver_summary: &DriverSummary) -> Result<()> {
    for row in driver_summary.data.iter() {
        println!("{row:?}");
    }
    Ok(())
}
