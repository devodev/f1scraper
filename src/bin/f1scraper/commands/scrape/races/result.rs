use std::collections::HashMap;

use crate::commands::scrape::ScrapeContext;
use crate::prelude::*;

use f1_scraper::format::Circuit;
use f1_scraper::parse::parse_races;
use f1_scraper::parse::RaceResultTable;
use f1_scraper::scrape::Scraper;
use log::debug;

use super::summary;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Only scrape the page for the provided year
    year: Option<u16>,

    /// The name of the Grand Prix
    circuit_name: Option<String>,

    /// Minimim year to use when scraping race pages
    #[arg(long, default_value_t = 1950)]
    year_min: u16,

    /// Maximum year to use when scraping race pages
    #[arg(long, default_value_t = 2023)]
    year_max: u16,
}

pub fn process(scrape_ctx: ScrapeContext, args: Args) -> Result<()> {
    let mut year_min = args.year_min;
    let mut year_max = args.year_max;
    if let Some(year) = args.year {
        year_min = year;
        year_max = year;
    }

    for year in year_min..=year_max {
        let summary = summary::query_and_parse(&scrape_ctx.scraper, year)?;

        // index circuits
        let mut hm = HashMap::new();
        for gp in &summary.data {
            let circuit = gp.circuit().with_context(|| {
                format!(
                    "obtain circuit infos from summary data (circuit: `{}`)",
                    gp.grand_prix
                )
            })?;
            hm.insert(circuit.name.clone(), circuit);
        }
        debug!("circuit index: {:?}", hm);

        let mut circuits: Vec<_> = hm.values().collect();
        if let Some(circuit_name) = &args.circuit_name {
            let circuit = hm.get(circuit_name).ok_or(anyhow::anyhow!(
                "find grand prix for year `{}` with name: {}",
                year,
                circuit_name
            ))?;
            circuits = hm.values().filter(|c| c.name == circuit.name).collect();
        }
        for circuit in circuits {
            let race_result = query_and_parse(&scrape_ctx.scraper, year, circuit)?;
            print(&race_result)?
        }
    }
    Ok(())
}

struct RaceResultTarget {
    url: reqwest::Url,
}

impl RaceResultTarget {
    fn new(year: u16, circuit: &Circuit) -> Result<Self> {
        let circuit_idx = circuit.idx;
        let circuit_name = &circuit.name;
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

fn query_and_parse(scraper: &Scraper, year: u16, circuit: &Circuit) -> Result<RaceResultTable> {
    // create scrape target
    let target = RaceResultTarget::new(year, circuit)
        .with_context(|| format!("create scrape target: race result {}", year))?;
    // run scrape
    let html = scraper
        .scrape(target)
        .with_context(|| format!("scrape: race result {}", year))?;

    // parse html text as race result
    let race_result = parse_races(&html, year, circuit.name.clone())?;
    Ok(race_result)
}

fn print(race_result: &RaceResultTable) -> Result<()> {
    let circuit = race_result.circuit.clone().unwrap_or("-".to_string());
    let prefix = format!("[{}][{}]", race_result.year, circuit);
    println!("{} {:?}", prefix, race_result.headers);
    for row in race_result.data.iter() {
        println!("{} {:?}", prefix, row);
    }
    Ok(())
}
