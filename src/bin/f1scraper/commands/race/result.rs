use log::debug;
use std::collections::HashMap;

use f1scraper::scrape::{RaceResultTarget, Scraper};
use f1scraper::types::{Circuit, RaceResult};

use crate::commands::{ScrapeContext, YearFlags};
use crate::prelude::*;

use super::summary;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// The name of the Grand Prix
    circuit_name: Option<String>,

    #[command(flatten)]
    year_flags: YearFlags,
}

pub fn run(scrape_ctx: ScrapeContext, args: Args) -> Result<()> {
    let (year_min, year_max) = args.year_flags.min_max();

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

fn query_and_parse(scraper: &Scraper, year: u16, circuit: &Circuit) -> Result<RaceResult> {
    // create scrape target
    let target = RaceResultTarget::new(year, circuit)
        .with_context(|| format!("create scrape target: race result {year}"))?;
    // run scrape
    let html = scraper
        .scrape(target)
        .with_context(|| format!("scrape: race result {year}"))?;

    // parse html text as race result
    let race_result = RaceResult::parse(&html, year, &circuit.clone())?;
    Ok(race_result)
}

fn print(race_result: &RaceResult) -> Result<()> {
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
