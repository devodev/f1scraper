use log::debug;

use anyhow::{Context, Result};

pub trait ScrapeTarget {
    fn request(&self) -> reqwest::blocking::Request;
}

#[derive(Debug, Default)]
pub struct Scraper {
    client: reqwest::blocking::Client,
}

impl Scraper {
    pub fn new<C: Into<reqwest::blocking::Client>>(client: C) -> Self {
        Self {
            client: client.into(),
        }
    }

    pub fn scrape(&self, target: impl ScrapeTarget) -> Result<String> {
        let req = target.request();
        let url = &req.url().clone();

        debug!("[{}] Executing reqwest", url);
        let response = self
            .client
            .execute(req)
            .with_context(|| format!("execute scrape request: {url}"))?;

        debug!(
            "[{}] Response: {:?} {}",
            url,
            response.version(),
            response.status()
        );
        debug!("[{}] Headers: {:#?}", url, response.headers());

        // handle errors
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "request failed: {}: {}",
                response.status(),
                &response.text().unwrap_or("".to_string())
            ));
        }

        // return body
        let text = response
            .text()
            .with_context(|| format!("parse scrape response as text: {url}"))?;
        Ok(text)
    }
}
