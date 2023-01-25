use f1scraper::parse::race::{parse_summary, ParsedRaceSummary};
use f1scraper::scrape::{RaceResultSummaryTarget, Scraper};

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
        let summaries = query_and_parse(&scrape_ctx.scraper, year)?;
        print(&summaries)?;
        return Ok(());
    }

    // range of years
    for year in args.year_flags.year_min..=args.year_flags.year_max {
        let summaries = query_and_parse(&scrape_ctx.scraper, year)?;
        print(&summaries)?
    }
    Ok(())
}

pub fn query_and_parse(scraper: &Scraper, year: u16) -> Result<ParsedRaceSummary> {
    // create scrape target
    let target = RaceResultSummaryTarget::new(year)
        .with_context(|| format!("create scrape target: race result summary {year}"))?;
    // run scrape
    let html = scraper
        .scrape(target)
        .with_context(|| format!("scrape: race result summary {year}"))?;

    // parse html text as race summary
    let summaries = parse_summary(&html, year)?;
    Ok(summaries)
}

fn print(summaries: &ParsedRaceSummary) -> Result<()> {
    for row in summaries.data.iter() {
        println!("{row:?}");
    }
    Ok(())
}
