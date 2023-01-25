use f1scraper::parse::team::{parse_summary, ParsedTeamSummary};
use f1scraper::scrape::{Scraper, TeamResultSummaryTarget};

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

pub fn query_and_parse(scraper: &Scraper, year: u16) -> Result<ParsedTeamSummary> {
    // create scrape target
    let target = TeamResultSummaryTarget::new(year)
        .with_context(|| format!("create scrape target: team result summary {year}"))?;
    // run scrape
    let html = scraper
        .scrape(target)
        .with_context(|| format!("scrape: team result summary {year}"))?;

    // parse html text as team summary
    let summaries = parse_summary(&html, year)?;
    Ok(summaries)
}

fn print(summaries: &ParsedTeamSummary) -> Result<()> {
    for row in summaries.data.iter() {
        println!("{row:?}");
    }
    Ok(())
}
