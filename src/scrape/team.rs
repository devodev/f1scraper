use crate::prelude::*;
use crate::scrape::ScrapeTarget;
use crate::types::Team;

pub struct TeamResultSummaryTarget {
    url: reqwest::Url,
}

impl TeamResultSummaryTarget {
    pub fn new(year: u16) -> Result<Self> {
        let url = format!("https://www.formula1.com/en/results.html/{year}/team.html");
        let url = reqwest::Url::parse(&url).with_context(|| format!("parse url: {}", &url))?;
        Ok(Self { url })
    }
}

impl ScrapeTarget for TeamResultSummaryTarget {
    fn request(&self) -> reqwest::blocking::Request {
        reqwest::blocking::Request::new(reqwest::Method::GET, self.url.clone())
    }
}

pub struct TeamResultTarget {
    url: reqwest::Url,
}

impl TeamResultTarget {
    pub fn new(year: u16, team: &Team) -> Result<Self> {
        let team_name = &team.name;
        let url = format!("https://www.formula1.com/en/results.html/{year}/team/{team_name}.html");
        let url = reqwest::Url::parse(&url).with_context(|| format!("parse url: {}", &url))?;
        Ok(Self { url })
    }
}

impl ScrapeTarget for TeamResultTarget {
    fn request(&self) -> reqwest::blocking::Request {
        reqwest::blocking::Request::new(reqwest::Method::GET, self.url.clone())
    }
}
