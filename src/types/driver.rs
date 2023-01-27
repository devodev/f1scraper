use std::fmt::Debug;

use anyhow::Context;
use scraper::{ElementRef, Html};

use crate::parse::HtmlTable;
use crate::prelude::*;

use super::ScrapperHelper;

#[derive(Default, Debug)]
pub struct DriverResult {
    pub year: u16,
    pub driver: DriverFragment,
    pub data: Vec<DriverResultEntry>,
}

impl DriverResult {
    const TABLE_SELECTOR_STR: &str = "div.resultsarchive-wrapper>div.resultsarchive-content>div.table-wrap>table.resultsarchive-table";

    pub fn parse(html: &str, year: u16, driver: &DriverFragment) -> Result<Self> {
        // parse html
        let document = Html::parse_document(html);
        let document_root = document.root_element();

        // select table
        let table = HtmlTable::parse(&document_root, Self::TABLE_SELECTOR_STR)?;

        // parse content
        let data: Result<Vec<_>, _> = table.rows().map(|r| DriverResultEntry::parse(&r)).collect();
        let data = data.with_context(|| "parse table rows")?;

        Ok(Self {
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
        let helper = ScrapperHelper::new();

        let cols: Vec<_> = helper.table_cols(row).collect();
        if cols.len() != 5 {
            return Err(anyhow::anyhow!("invalid column count"));
        }

        let grand_prix = helper
            .link(&cols[0])
            .with_context(|| "column: grand_prix")?;
        let date = helper
            .inner_html(&cols[1])
            .with_context(|| "column: date")?;
        let car = helper.link(&cols[2]).with_context(|| "column: car")?;
        let pos = helper.inner_html(&cols[3]).with_context(|| "column: pos")?;
        let pts = helper.inner_html(&cols[4]).with_context(|| "column: pts")?;

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

    pub fn parse(html: &str, year: u16) -> Result<Self> {
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

        Ok(Self { year, data })
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
        let helper = ScrapperHelper::new();

        let cols: Vec<_> = helper.table_cols(row).collect();
        if cols.len() != 5 {
            return Err(anyhow::anyhow!("invalid column count"));
        }

        let pos = helper.inner_html(&cols[0]).with_context(|| "column: pos")?;
        let driver_col = helper
            .link_elem(&cols[1])
            .with_context(|| "column: driver")?;
        let url = helper.href(&driver_col).with_context(|| "column: driver")?;
        let driver = helper
            .join_spans(&driver_col)
            .with_context(|| "column: driver")?;
        let nationality = helper
            .inner_html(&cols[2])
            .with_context(|| "column: nationality")?;
        let car = helper.link(&cols[3]).with_context(|| "column: car")?;
        let pts = helper.inner_html(&cols[4]).with_context(|| "column: pts")?;

        Ok(Self {
            pos,
            url,
            driver,
            nationality,
            car,
            pts,
        })
    }

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
