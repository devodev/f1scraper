use log::debug;
use std::collections::HashMap;

use f1scraper::parse::race::{parse_result, ParsedRaceResult};
use f1scraper::scrape::{RaceResultTarget, Scraper};
use f1scraper::types::Circuit;

use crate::commands::ScrapeContext;
use crate::{prelude::*, YearFlags};

use super::summary;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// The name of the Grand Prix
    circuit_name: Option<String>,

    #[command(flatten)]
    year_flags: YearFlags,
}

pub fn process(scrape_ctx: ScrapeContext, args: Args) -> Result<()> {
    let mut year_min = args.year_flags.year_min;
    let mut year_max = args.year_flags.year_max;
    if let Some(year) = args.year_flags.year {
        year_min = year;
        year_max = year;
    }

    for year in year_min..=year_max {
        // query summary to obtain the list of available circuits
        let summary = summary::query_and_parse(&scrape_ctx.scraper, year)?;

        // retrieve circuits
        // index by name
        let mut circuit_by_name = HashMap::new();
        let mut circuit_by_display_name = HashMap::new();
        for gp in &summary.data {
            let circuit = gp.circuit().with_context(|| {
                format!(
                    "obtain circuit infos from summary data (circuit: `{}`)",
                    gp.grand_prix
                )
            })?;
            circuit_by_name.insert(circuit.name.clone().trim().to_lowercase(), circuit.clone());
            circuit_by_display_name
                .insert(circuit.display_name.clone().trim().to_lowercase(), circuit);
        }
        debug!("circuit index: {:?}", circuit_by_name);

        // all circuits
        let mut circuits: Vec<_> = circuit_by_name.values().collect();

        // if argument passed, filter by circuit name
        if let Some(circuit_name) = &args.circuit_name {
            let circuit_name = &circuit_name.trim().to_lowercase();
            let circuit_name_indexes = vec![&circuit_by_name, &circuit_by_display_name];
            let circuit = circuit_name_indexes
                .iter()
                .filter_map(|index| index.get(circuit_name))
                .next()
                .ok_or(anyhow::anyhow!(
                    "find grand prix for year `{}` with name: {}",
                    year,
                    circuit_name
                ))?;
            circuits = vec![circuit];
        }
        for circuit in circuits {
            let race_result = query_and_parse(&scrape_ctx.scraper, year, circuit)?;
            print(&race_result)?
        }
    }
    Ok(())
}

fn query_and_parse(scraper: &Scraper, year: u16, circuit: &Circuit) -> Result<ParsedRaceResult> {
    // create scrape target
    let target = RaceResultTarget::new(year, circuit)
        .with_context(|| format!("create scrape target: race result {year}"))?;
    // run scrape
    let html = scraper
        .scrape(target)
        .with_context(|| format!("scrape: race result {year}"))?;

    // parse html text as race result
    let race_result = parse_result(&html, year, &circuit.clone())?;
    Ok(race_result)
}

fn print(race_result: &ParsedRaceResult) -> Result<()> {
    let default_value = "-".to_string();
    let mut circuit_name = &race_result.circuit.name;
    if circuit_name.is_empty() {
        circuit_name = &default_value;
    }
    let mut circuit_display_name = &race_result.circuit.display_name;
    if circuit_display_name.is_empty() {
        circuit_display_name = &default_value;
    }

    let prefix = format!(
        "[{}][{} ({})]",
        race_result.year, circuit_display_name, circuit_name
    );
    for row in race_result.data.iter() {
        println!("{prefix} {row:?}");
    }
    Ok(())
}
