use f1scraper::scrape::{FastestLapResultSummaryTarget, Scraper};
use f1scraper::types::FastestLapSummary;

use crate::commands::{ScrapeContext, YearFlags};
use crate::prelude::*;

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

pub fn query_and_parse(scraper: &Scraper, year: u16) -> Result<FastestLapSummary> {
    // create scrape target
    let target = FastestLapResultSummaryTarget::new(year)
        .with_context(|| format!("create scrape target: fastest_lap result summary {year}"))?;
    // run scrape
    let html = scraper
        .scrape(target)
        .with_context(|| format!("scrape: fastest_lap result summary {year}"))?;
    // parse html text as fastest_lap summary
    let summaries = FastestLapSummary::parse(&html, year)?;
    Ok(summaries)
}

fn print(summaries: &FastestLapSummary) -> Result<()> {
    for row in summaries.data.iter() {
        println!("{row:?}");
    }
    Ok(())
}
