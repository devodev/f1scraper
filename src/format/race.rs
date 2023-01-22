use std::fmt::Debug;

use crate::prelude::*;

#[derive(Default, Debug)]
pub struct Race {
    pub name: String,
    pub circuit: String,
    pub date: String,

    pub race_results: Vec<Table<RaceResultHeaders, RaceResultData>>,
    pub fastest_laps: Vec<FastestLap>,
    pub qualifyings: Vec<Qualifying>,
}

#[derive(Default, Debug)]
pub struct Table<Headers: Debug, Data: Debug> {
    pub year: u16,
    pub circuit: Option<Circuit>,
    pub headers: Headers,
    pub data: Vec<Data>,
}

impl<Headers: Debug, Data: Debug> Table<Headers, Data> {
    pub fn new<U: Into<u16>, D: Into<Vec<Data>>>(year: U, headers: Headers, data: D) -> Self {
        Self {
            year: year.into(),
            circuit: None,
            headers: headers,
            data: data.into(),
        }
    }

    pub fn with_circuit<S: Into<Circuit>>(mut self, circuit: S) -> Self {
        self.circuit = Some(circuit.into());
        self
    }
}

#[derive(Default, Debug)]
pub struct RaceResultData {
    pub pos: String,
    pub no: String,
    pub driver: String,
    pub car: String,
    pub laps: String,
    pub time_retired: String,
    pub pts: String,
}

#[derive(Default, Debug)]
pub struct RaceResultHeaders {
    pub pos: String,
    pub no: String,
    pub driver: String,
    pub car: String,
    pub laps: String,
    pub time_retired: String,
    pub pts: String,
}

#[derive(Default, Debug)]
pub struct RaceResultSummaryHeaders {
    pub grand_prix: String,
    pub date: String,
    pub winner: String,
    pub car: String,
    pub laps: String,
    pub time: String,
}

#[derive(Default, Debug)]
pub struct RaceResultSummaryData {
    pub grand_prix: String,
    pub url: String,
    pub date: String,
    pub winner: String,
    pub car: String,
    pub laps: String,
    pub time: String,
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

impl RaceResultSummaryData {
    pub fn circuit(&self) -> Result<Circuit> {
        // Example:
        //   /en/results.html/1950/races/100/italy/race-result.html
        let tokens: Vec<_> = self.url.split("/").skip(5).take(2).collect();
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
