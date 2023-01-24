use crate::prelude::*;
use crate::scrape::ScrapeTarget;
use crate::types::DriverFragment;

pub struct DriverResultSummaryTarget {
    url: reqwest::Url,
}

impl DriverResultSummaryTarget {
    pub fn new(year: u16) -> Result<Self> {
        let url = format!("https://www.formula1.com/en/results.html/{year}/drivers.html");
        let url = reqwest::Url::parse(&url).with_context(|| format!("parse url: {}", &url))?;
        Ok(Self { url })
    }
}

impl ScrapeTarget for DriverResultSummaryTarget {
    fn request(&self) -> reqwest::blocking::Request {
        reqwest::blocking::Request::new(reqwest::Method::GET, self.url.clone())
    }
}

pub struct DriverResultTarget {
    url: reqwest::Url,
}

impl DriverResultTarget {
    pub fn new(year: u16, fragment: &DriverFragment) -> Result<Self> {
        let fragment_id = &fragment.id;
        let fragment_name = &fragment.name;
        let url = format!("https://www.formula1.com/en/results.html/{year}/drivers/{fragment_id}/{fragment_name}.html");
        let url = reqwest::Url::parse(&url).with_context(|| format!("parse url: {}", &url))?;
        Ok(Self { url })
    }
}

impl ScrapeTarget for DriverResultTarget {
    fn request(&self) -> reqwest::blocking::Request {
        reqwest::blocking::Request::new(reqwest::Method::GET, self.url.clone())
    }
}
