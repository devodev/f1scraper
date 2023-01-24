use crate::prelude::*;
use crate::types::Circuit;

pub struct RaceResultSummaryTarget {
    url: reqwest::Url,
}

impl RaceResultSummaryTarget {
    pub fn new(year: u16) -> Result<Self> {
        let url = format!("https://www.formula1.com/en/results.html/{year}/races.html");
        let url = reqwest::Url::parse(&url).with_context(|| format!("parse url: {}", &url))?;
        Ok(Self { url })
    }
}

impl crate::scrape::ScrapeTarget for RaceResultSummaryTarget {
    fn request(&self) -> reqwest::blocking::Request {
        reqwest::blocking::Request::new(reqwest::Method::GET, self.url.clone())
    }
}

pub struct RaceResultTarget {
    url: reqwest::Url,
}

impl RaceResultTarget {
    pub fn new(year: u16, circuit: &Circuit) -> Result<Self> {
        let circuit_idx = circuit.idx;
        let circuit_name = &circuit.name;
        let url = format!("https://www.formula1.com/en/results.html/{year}/races/{circuit_idx}/{circuit_name}/race-result.html");
        let url = reqwest::Url::parse(&url).with_context(|| format!("parse url: {}", &url))?;
        Ok(Self { url })
    }
}

impl crate::scrape::ScrapeTarget for RaceResultTarget {
    fn request(&self) -> reqwest::blocking::Request {
        reqwest::blocking::Request::new(reqwest::Method::GET, self.url.clone())
    }
}
