use f1scraper::parse::driver::{parse_summary, ParsedDriverSummary};
use f1scraper::scrape::{DriverResultSummaryTarget, Scraper};

use crate::commands::ScrapeContext;
use crate::{prelude::*, YearFlags};

#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    year_flags: YearFlags,
}

pub fn process(scrape_ctx: ScrapeContext, args: Args) -> Result<()> {
    // exact year takes precedence
    if let Some(year) = args.year_flags.year {
        let driver_summary = query_and_parse(&scrape_ctx.scraper, year)?;
        print(&driver_summary)?;
        return Ok(());
    }

    // range of years
    for year in args.year_flags.year_min..=args.year_flags.year_max {
        let driver_summary = query_and_parse(&scrape_ctx.scraper, year)?;
        print(&driver_summary)?
    }
    Ok(())
}

pub fn query_and_parse(scraper: &Scraper, year: u16) -> Result<ParsedDriverSummary> {
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

fn print(driver_summary: &ParsedDriverSummary) -> Result<()> {
    for row in driver_summary.data.iter() {
        println!("{row:?}");
    }
    Ok(())
}
