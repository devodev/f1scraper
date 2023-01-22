use anyhow::{Context, Result};

pub trait ScrapeTarget {
    fn request(&self) -> reqwest::blocking::Request;
}

#[derive(Debug, Default)]
pub struct Scraper {
    client: reqwest::blocking::Client,
}

impl Scraper {
    pub fn new(client: reqwest::blocking::Client) -> Self {
        Self { client }
    }

    pub fn scrape(&self, target: impl ScrapeTarget) -> Result<String> {
        let req = target.request();
        let url = &req.url().clone();
        let text = self
            .client
            .execute(req)
            .with_context(|| format!("execute scrape request: {}", url))?
            .text()
            .with_context(|| format!("parse scrape response as text: {}", url))?;
        Ok(text)
    }
}
