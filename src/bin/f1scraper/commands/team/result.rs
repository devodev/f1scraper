use log::debug;
use std::collections::HashMap;

use f1scraper::parse::team::{parse_result, ParsedTeamResult};
use f1scraper::scrape::{Scraper, TeamResultTarget};
use f1scraper::types::Team;

use crate::commands::ScrapeContext;
use crate::{prelude::*, YearFlags};

use super::summary;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// The name of the Grand Prix
    team_name: Option<String>,

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
        // query summary to obtain the list of available teams
        let summary = summary::query_and_parse(&scrape_ctx.scraper, year)?;

        // retrieve teams
        // index by name
        let mut team_by_name = HashMap::new();
        let mut team_by_display_name = HashMap::new();
        for gp in &summary.data {
            let team = gp.team().with_context(|| {
                format!("obtain team infos from summary data (team: `{}`)", gp.team)
            })?;
            team_by_name.insert(team.name.clone().trim().to_lowercase(), team.clone());
            team_by_display_name.insert(team.display_name.clone().trim().to_lowercase(), team);
        }
        debug!("team index: {:?}", team_by_name);

        // all teams
        let mut teams: Vec<_> = team_by_name.values().collect();

        // if argument passed, filter by team name
        if let Some(team_name) = &args.team_name {
            let team_name = &team_name.trim().to_lowercase();
            let team_name_indexes = vec![&team_by_name, &team_by_display_name];
            let team = team_name_indexes
                .iter()
                .filter_map(|index| index.get(team_name))
                .next()
                .ok_or(anyhow::anyhow!(
                    "find team for year `{}` with name: {}",
                    year,
                    team_name
                ))?;
            teams = vec![team];
        }
        for team in teams {
            let team_result = query_and_parse(&scrape_ctx.scraper, year, team)?;
            print(&team_result)?
        }
    }
    Ok(())
}

fn query_and_parse(scraper: &Scraper, year: u16, team: &Team) -> Result<ParsedTeamResult> {
    // create scrape target
    let target = TeamResultTarget::new(year, team)
        .with_context(|| format!("create scrape target: team result {year}"))?;
    // run scrape
    let html = scraper
        .scrape(target)
        .with_context(|| format!("scrape: team result {year}"))?;

    // parse html text as team result
    let team_result = parse_result(&html, year, &team.clone())?;
    Ok(team_result)
}

fn print(team_result: &ParsedTeamResult) -> Result<()> {
    let default_value = "-".to_string();
    let mut team_name = &team_result.team.name;
    if team_name.is_empty() {
        team_name = &default_value;
    }
    let mut team_display_name = &team_result.team.display_name;
    if team_display_name.is_empty() {
        team_display_name = &default_value;
    }

    let prefix = format!(
        "[{}][{} ({})]",
        team_result.year, team_display_name, team_name
    );
    for row in team_result.data.iter() {
        println!("{prefix} {row:?}");
    }
    Ok(())
}
