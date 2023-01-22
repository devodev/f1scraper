use crate::commands::scrape::ScrapeContext;
use crate::prelude::*;

use f1_scraper::parse::parse_races;
use f1_scraper::parse::RaceResultTable;
use f1_scraper::scrape::Scraper;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Only scrape the page for the provided year
    #[arg(short, long)]
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
        let race_result = query_and_parse(&scrape_ctx.scraper, year)?;
        print(&race_result)?;
        return Ok(());
    }

    // range of years
    for year in args.year_min..=args.year_max {
        let race_result = query_and_parse(&scrape_ctx.scraper, year)?;
        print(&race_result)?
    }
    Ok(())
}

struct RaceResultTarget {
    url: reqwest::Url,
}

impl RaceResultTarget {
    fn new(year: u16) -> Result<Self> {
        let circuit_idx = "94";
        let circuit_name = "great-britain";
        let url = format!("https://www.formula1.com/en/results.html/{year}/races/{circuit_idx}/{circuit_name}/race-result.html");
        let url = reqwest::Url::parse(&url).with_context(|| format!("parse url: {}", &url))?;
        Ok(Self { url })
    }
}

impl f1_scraper::scrape::ScrapeTarget for RaceResultTarget {
    fn request(&self) -> reqwest::blocking::Request {
        reqwest::blocking::Request::new(reqwest::Method::GET, self.url.clone())
    }
}

fn query_and_parse(scraper: &Scraper, year: u16) -> Result<RaceResultTable> {
    // create scrape target
    let target = RaceResultTarget::new(year)
        .with_context(|| format!("create scrape target: race result {}", year))?;
    // run scrape
    let html = scraper
        .scrape(target)
        .with_context(|| format!("scrape: race result {}", year))?;

    // parse html text as race result
    let race_result = parse_races(format!("{year}"), &html)?;
    Ok(race_result)
}

fn print(race_result: &RaceResultTable) -> Result<()> {
    println!("{:?}", race_result.headers);
    for row in race_result.data.iter() {
        println!("{:?}", row);
    }
    Ok(())
}
