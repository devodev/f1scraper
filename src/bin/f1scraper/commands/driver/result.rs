use log::debug;
use std::collections::HashMap;

use crate::commands::ScrapeContext;
use crate::{prelude::*, YearFlags};

use f1_scraper::parse::driver::{parse_result, DriverResultTable};
use f1_scraper::scrape::{DriverResultTarget, Scraper};
use f1_scraper::types::DriverFragment;

use super::summary;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// The name of the Grand Prix
    driver_name: Option<String>,

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
        // query summary to obtain the list of available drivers
        let summary = summary::query_and_parse(&scrape_ctx.scraper, year)?;

        // retrieve drivers
        // index by name
        let mut driver_by_name = HashMap::new();
        let mut driver_by_display_name = HashMap::new();
        for gp in &summary.data {
            let driver = gp.driver().with_context(|| {
                format!(
                    "obtain driver infos from summary data (driver: `{}`)",
                    gp.driver
                )
            })?;
            driver_by_name.insert(driver.name.clone().trim().to_lowercase(), driver.clone());
            driver_by_display_name
                .insert(driver.display_name.clone().trim().to_lowercase(), driver);
        }
        debug!("driver index: {:?}", driver_by_name);

        // all drivers
        let mut drivers: Vec<_> = driver_by_name.values().collect();

        // if argument passed, filter by driver name
        if let Some(driver_name) = &args.driver_name {
            let driver_name = &driver_name.trim().to_lowercase();
            let driver_name_indexes = vec![&driver_by_name, &driver_by_display_name];
            let driver = driver_name_indexes
                .iter()
                .filter_map(|index| index.get(driver_name))
                .next()
                .ok_or(anyhow::anyhow!(
                    "find grand prix for year `{}` with name: {}",
                    year,
                    driver_name
                ))?;
            drivers = vec![driver];
        }
        for driver in drivers {
            let driver_result = query_and_parse(&scrape_ctx.scraper, year, driver)?;
            print(&driver_result)?
        }
    }
    Ok(())
}

fn query_and_parse(
    scraper: &Scraper,
    year: u16,
    driver: &DriverFragment,
) -> Result<DriverResultTable> {
    // create scrape target
    let target = DriverResultTarget::new(year, driver)
        .with_context(|| format!("create scrape target: driver result {year}"))?;
    // run scrape
    let html = scraper
        .scrape(target)
        .with_context(|| format!("scrape: driver result {year}"))?;

    // parse html text as driver result
    let driver_result = parse_result(&html, year, &driver.clone())?;
    Ok(driver_result)
}

fn print(driver_result: &DriverResultTable) -> Result<()> {
    let default_value = "-".to_string();
    let driver_name = driver_result
        .driver
        .as_ref()
        .map(|c| &c.name)
        .unwrap_or(&default_value);
    let driver_display_name = driver_result
        .driver
        .as_ref()
        .map(|c| &c.display_name)
        .unwrap_or(&default_value);
    let prefix = format!(
        "[{}][{} ({})]",
        driver_result.year, driver_display_name, driver_name
    );
    println!("{} {:?}", prefix, driver_result.headers);
    for row in driver_result.data.iter() {
        println!("{prefix} {row:?}");
    }
    Ok(())
}