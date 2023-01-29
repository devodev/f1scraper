use crate::prelude::*;
use crate::scrape::ScrapeTarget;

pub struct FastestLapResultSummaryTarget {
    url: reqwest::Url,
}

impl FastestLapResultSummaryTarget {
    pub fn new(year: u16) -> Result<Self> {
        let url = format!("https://www.formula1.com/en/results.html/{year}/fastest-laps.html");
        let url = reqwest::Url::parse(&url).with_context(|| format!("parse url: {}", &url))?;
        Ok(Self { url })
    }
}

impl ScrapeTarget for FastestLapResultSummaryTarget {
    fn request(&self) -> reqwest::blocking::Request {
        reqwest::blocking::Request::new(reqwest::Method::GET, self.url.clone())
    }
}
