use std::fmt::Debug;

use crate::prelude::*;

#[derive(Default, Debug)]
pub struct RaceResult {
    pub pos: String,
    pub no: String,
    pub driver: String,
    pub car: String,
    pub laps: String,
    pub time_retired: String,
    pub pts: String,
}

#[derive(Default, Debug)]
pub struct RaceSummary {
    pub grand_prix: String,
    pub url: String,
    pub date: String,
    pub winner: String,
    pub car: String,
    pub laps: String,
    pub time: String,
}

impl RaceSummary {
    pub fn circuit(&self) -> Result<Circuit> {
        // Example:
        //   /en/results.html/1950/races/100/italy/race-result.html
        let tokens: Vec<_> = self.url.split('/').skip(5).take(2).collect();
        if tokens.len() != 2 {
            return Err(anyhow::anyhow!(
                "can't parse url: invalid format: {}",
                self.url
            ));
        }

        let idx_token = tokens[0];
        let idx = idx_token.parse::<u16>().with_context(|| {
            format!(
                "parse circuit index from url (token: `{}`): `{}`",
                idx_token, self.url
            )
        })?;
        let name = tokens[1];

        Ok(Circuit::new(idx, name, &self.grand_prix))
    }
}

#[derive(Default, Debug, Clone)]
pub struct Circuit {
    pub idx: u16,
    pub name: String,
    pub display_name: String,
}

impl Circuit {
    fn new<S: Into<String>>(idx: u16, name: S, display_name: S) -> Self {
        Self {
            idx,
            name: name.into(),
            display_name: display_name.into(),
        }
    }
}

#[derive(Default, Debug)]
pub struct FastestLap {
    pub pos: String,
    pub no: String,
    pub driver: String,
    pub car: String,
    pub lap: String,
    pub time: String,
}

#[derive(Default, Debug)]
pub struct Qualifying {
    pub pos: String,
    pub no: String,
    pub driver: String,
    pub car: String,
    pub time: String,
}
