use std::fmt::Debug;

use crate::prelude::*;

use super::Table;

#[derive(Default, Debug)]
pub struct Driver {
    pub name: String,
    pub circuit: String,
    pub date: String,

    pub race_results: Vec<Table<DriverResultHeaders, DriverResultData>>,
}

#[derive(Default, Debug)]
pub struct DriverResultData {
    pub grand_prix: String,
    pub date: String,
    pub car: String,
    pub pos: String,
    pub pts: String,
}

#[derive(Default, Debug)]
pub struct DriverResultHeaders {
    pub grand_prix: String,
    pub date: String,
    pub car: String,
    pub pos: String,
    pub pts: String,
}

#[derive(Default, Debug)]
pub struct DriverResultSummaryHeaders {
    pub pos: String,
    pub driver: String,
    pub nationality: String,
    pub car: String,
    pub pts: String,
}

#[derive(Default, Debug)]
pub struct DriverResultSummaryData {
    pub pos: String,
    pub url: String,
    pub driver: String,
    pub nationality: String,
    pub car: String,
    pub pts: String,
}

#[derive(Default, Debug, Clone)]
pub struct DriverFragment {
    pub id: String,
    pub name: String,
    pub display_name: String,
}

impl DriverFragment {
    fn new<S: Into<String>>(id: S, name: S, display_name: S) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            display_name: display_name.into(),
        }
    }
}

impl DriverResultSummaryData {
    pub fn driver(&self) -> Result<DriverFragment> {
        // Example:
        //   /en/results.html/1950/drivers/NINFAR01/nino-farina.html
        let tokens: Vec<_> = self.url.split('/').skip(5).take(2).collect();
        if tokens.len() != 2 {
            return Err(anyhow::anyhow!(
                "can't parse url: invalid format: {}",
                self.url
            ));
        }

        let id = tokens[0];
        let name = tokens[1].trim_end_matches(".html");

        Ok(DriverFragment::new(id, name, &self.driver))
    }
}
