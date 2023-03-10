use f1scraper::scrape::{RaceResultSummaryTarget, Scraper};
use f1scraper::types::RaceSummary;

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

pub fn query_and_parse(scraper: &Scraper, year: u16) -> Result<RaceSummary> {
    // create scrape target
    let target = RaceResultSummaryTarget::new(year)
        .with_context(|| format!("create scrape target: race result summary {year}"))?;
    // run scrape
    let html = scraper
        .scrape(target)
        .with_context(|| format!("scrape: race result summary {year}"))?;
    // parse html text as race summary
    let summaries = RaceSummary::parse(&html, year)?;
    Ok(summaries)
}

fn print(summaries: &RaceSummary) -> Result<()> {
    for row in summaries.data.iter() {
        println!("{row:?}");
    }
    Ok(())
}
