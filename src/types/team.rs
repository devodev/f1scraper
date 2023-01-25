use std::fmt::Debug;

use crate::prelude::*;

#[derive(Default, Debug)]
pub struct TeamResult {
    pub grand_prix: String,
    pub date: String,
    pub pts: String,
}

#[derive(Default, Debug)]
pub struct TeamSummary {
    pub pos: String,
    pub url: String,
    pub team: String,
    pub pts: String,
}

impl TeamSummary {
    pub fn team(&self) -> Result<Team> {
        // Example:
        //   /en/results.html/1950/team/alfa_romeo_ferrari.html
        let tokens: Vec<_> = self.url.split('/').skip(5).take(1).collect();
        if tokens.len() != 1 {
            return Err(anyhow::anyhow!(
                "can't parse url: invalid format: {}",
                self.url
            ));
        }

        let name = tokens[0].trim_end_matches(".html");

        Ok(Team::new(name, &self.team))
    }
}

#[derive(Default, Debug, Clone)]
pub struct Team {
    pub name: String,
    pub display_name: String,
}

impl Team {
    fn new<S: Into<String>>(name: S, display_name: S) -> Self {
        Self {
            name: name.into(),
            display_name: display_name.into(),
        }
    }
}
