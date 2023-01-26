use std::fmt::Debug;

use anyhow::Context;
use scraper::{ElementRef, Html, Selector};
use selectors::attr::CaseSensitivity;

use crate::parse::{next_inner_html, HtmlTable};
use crate::prelude::*;

#[derive(Default, Debug)]
pub struct DriverResult {
    pub year: u16,
    pub driver: DriverFragment,
    pub data: Vec<DriverResultEntry>,
}

impl DriverResult {
    const TABLE_SELECTOR_STR: &str = "div.resultsarchive-wrapper>div.resultsarchive-content>div.table-wrap>table.resultsarchive-table";

    pub fn parse(html: &str, year: u16, driver: &DriverFragment) -> Result<DriverResult> {
        // parse html
        let document = Html::parse_document(html);
        let document_root = document.root_element();

        // select table
        let table = HtmlTable::parse(&document_root, Self::TABLE_SELECTOR_STR)?;

        // parse content
        let data: Result<Vec<_>, _> = table.rows().map(|r| DriverResultEntry::parse(&r)).collect();
        let data = data.with_context(|| "parse table rows")?;

        Ok(DriverResult {
            year,
            driver: driver.clone(),
            data,
        })
    }
}

#[derive(Default, Debug)]
pub struct DriverResultEntry {
    pub grand_prix: String,
    pub date: String,
    pub car: String,
    pub pos: String,
    pub pts: String,
}

impl DriverResultEntry {
    pub fn parse(row: &ElementRef) -> Result<Self> {
        let a = Selector::parse("a").unwrap();
        let td = Selector::parse("td").unwrap();

        let mut cols = row.select(&td).filter(|row| {
            !row.value()
                .has_class("limiter", CaseSensitivity::AsciiCaseInsensitive)
        });

        let grand_prix = cols
            .next()
            .ok_or(anyhow::anyhow!("expected column: grand_prix"))?
            .select(&a)
            .map(|x| x.inner_html())
            .next()
            .ok_or(anyhow::anyhow!(
                "expected column: grand_prix in <a> element"
            ))?
            .trim()
            .to_string();
        let date = next_inner_html(&mut cols).with_context(|| "column: date")?;
        let car = cols
            .next()
            .ok_or(anyhow::anyhow!("expected column: car"))?
            .select(&a)
            .map(|x| x.inner_html())
            .next()
            .ok_or(anyhow::anyhow!("expected column: car in <a> element"))?
            .trim()
            .to_string();
        let pos = next_inner_html(&mut cols).with_context(|| "column: pos")?;
        let pts = next_inner_html(&mut cols).with_context(|| "column:pts")?;

        Ok(Self {
            grand_prix,
            date,
            pos,
            car,
            pts,
        })
    }
}

#[derive(Default, Debug)]
pub struct DriverSummary {
    pub year: u16,
    pub data: Vec<DriverSummaryEntry>,
}

impl DriverSummary {
    const TABLE_SELECTOR_STR: &str =
    "div.resultsarchive-wrapper>div.resultsarchive-content>div.table-wrap>table.resultsarchive-table";

    pub fn parse(html: &str, year: u16) -> Result<DriverSummary> {
        // parse html
        let document = Html::parse_document(html);
        let document_root = document.root_element();

        // select table
        let table = HtmlTable::parse(&document_root, Self::TABLE_SELECTOR_STR)?;

        // parse rows
        let data: Result<Vec<_>, _> = table
            .rows()
            .map(|r| DriverSummaryEntry::parse(&r))
            .collect();
        let data = data.with_context(|| "parse table rows")?;

        Ok(DriverSummary { year, data })
    }
}

#[derive(Default, Debug)]
pub struct DriverSummaryEntry {
    pub pos: String,
    pub url: String,
    pub driver: String,
    pub nationality: String,
    pub car: String,
    pub pts: String,
}

impl DriverSummaryEntry {
    fn parse(row: &ElementRef) -> Result<Self> {
        let a = Selector::parse("a").unwrap();
        let td = Selector::parse("td").unwrap();
        let span = Selector::parse("span").unwrap();

        let mut cols = row.select(&td).filter(|row| {
            !row.value()
                .has_class("limiter", CaseSensitivity::AsciiCaseInsensitive)
        });

        let pos = next_inner_html(&mut cols).with_context(|| "column: pos")?;
        let driver_col = cols
            .next()
            .ok_or(anyhow::anyhow!("expected table column: driver"))?
            .select(&a)
            .next()
            .ok_or(anyhow::anyhow!(
                "expected a element on table column: driver"
            ))?;

        let url = driver_col
            .value()
            .attr("href")
            .ok_or(anyhow::anyhow!(
                "expected a element to contain url on column: driver"
            ))?
            .trim()
            .to_string();

        let driver = driver_col
            .select(&span)
            .map(|x| x.inner_html())
            .collect::<Vec<String>>()
            .join(" ")
            .trim()
            .to_string();
        let nationality = next_inner_html(&mut cols).with_context(|| "column: nationality")?;
        let car = cols
            .next()
            .ok_or(anyhow::anyhow!("column: car"))?
            .select(&a)
            .next()
            .ok_or(anyhow::anyhow!("column: car in <a> element"))?
            .inner_html()
            .trim()
            .to_string();
        let pts = next_inner_html(&mut cols).with_context(|| "column: pts")?;

        Ok(Self {
            pos,
            url,
            driver,
            nationality,
            car,
            pts,
        })
    }
}

impl DriverSummaryEntry {
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
