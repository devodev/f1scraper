use crate::commands::ScrapeContext;
use crate::prelude::*;

use f1_scraper::parse::race::{parse_summary, RaceResultSummaryTable};
use f1_scraper::scrape::{RaceResultSummaryTarget, Scraper};

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Only scrape the page for the provided year
    year: Option<u16>,

    /// Minimim year to use when scraping race pages
    #[arg(long, default_value_t = 1950)]
    year_min: u16,

    /// Maximum year to use when scraping race pages
    #[arg(long, default_value_t = 2023)]
    year_max: u16,
}

pub fn process(scrape_ctx: ScrapeContext, args: Args) -> Result<()> {
    // exact year takes precedence
    if let Some(year) = args.year {
        let race_summary = query_and_parse(&scrape_ctx.scraper, year)?;
        print(&race_summary)?;
        return Ok(());
    }

    // range of years
    for year in args.year_min..=args.year_max {
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
