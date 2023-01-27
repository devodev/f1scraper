use f1scraper::scrape::{Scraper, TeamResultSummaryTarget};
use f1scraper::types::TeamSummary;

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

pub fn query_and_parse(scraper: &Scraper, year: u16) -> Result<TeamSummary> {
    // create scrape target
    let target = TeamResultSummaryTarget::new(year)
        .with_context(|| format!("create scrape target: team result summary {year}"))?;
    // run scrape
    let html = scraper
        .scrape(target)
        .with_context(|| format!("scrape: team result summary {year}"))?;
    // parse html text as team summary
    let summaries = TeamSummary::parse(&html, year)?;
    Ok(summaries)
}

fn print(summaries: &TeamSummary) -> Result<()> {
    for row in summaries.data.iter() {
        println!("{row:?}");
    }
    Ok(())
}
