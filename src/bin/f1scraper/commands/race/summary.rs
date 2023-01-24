use crate::commands::ScrapeContext;
use crate::{prelude::*, YearFlags};

use f1_scraper::parse::race::{parse_summary, RaceResultSummaryTable};
use f1_scraper::scrape::{RaceResultSummaryTarget, Scraper};

#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    year_flags: YearFlags,
}

pub fn process(scrape_ctx: ScrapeContext, args: Args) -> Result<()> {
    // exact year takes precedence
    if let Some(year) = args.year_flags.year {
        let race_summary = query_and_parse(&scrape_ctx.scraper, year)?;
        print(&race_summary)?;
        return Ok(());
    }

    // range of years
    for year in args.year_flags.year_min..=args.year_flags.year_max {
        let race_summary = query_and_parse(&scrape_ctx.scraper, year)?;
        print(&race_summary)?
    }
    Ok(())
}

pub fn query_and_parse(scraper: &Scraper, year: u16) -> Result<RaceResultSummaryTable> {
    // create scrape target
    let target = RaceResultSummaryTarget::new(year)
        .with_context(|| format!("create scrape target: race result summary {year}"))?;
    // run scrape
    let html = scraper
        .scrape(target)
        .with_context(|| format!("scrape: race result summary {year}"))?;

    // parse html text as race summary
    let race_summary = parse_summary(&html, year)?;
    Ok(race_summary)
}

fn print(race_summary: &RaceResultSummaryTable) -> Result<()> {
    println!("{:?}", race_summary.headers);
    for row in race_summary.data.iter() {
        println!("{row:?}");
    }
    Ok(())
}
