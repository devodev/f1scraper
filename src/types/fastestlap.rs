use std::fmt::Debug;

use anyhow::Context;
use scraper::{ElementRef, Html};

use crate::parse::HtmlTable;
use crate::prelude::*;

use super::ScrapperHelper;

#[derive(Default, Debug)]
pub struct FastestLapSummary {
    pub year: u16,
    pub data: Vec<FastestLapSummaryEntry>,
}

impl FastestLapSummary {
    const TABLE_SELECTOR_STR: &str =
        "div.resultsarchive-content>div.table-wrap>table.resultsarchive-table";

    pub fn parse(html: &str, year: u16) -> Result<FastestLapSummary> {
        // parse html
        let document = Html::parse_document(html);
        let document_root = document.root_element();

        // select table
        let table = HtmlTable::parse(&document_root, Self::TABLE_SELECTOR_STR)?;

        // parse rows
        let data: Result<Vec<_>, _> = table
            .rows()
            .map(|r| FastestLapSummaryEntry::parse(&r))
            .collect();
        let data = data.with_context(|| "parse table rows")?;

        Ok(FastestLapSummary { year, data })
    }
}

#[derive(Default, Debug)]
pub struct FastestLapSummaryEntry {
    pub grand_prix: String,
    pub driver: String,
    pub car: String,
    pub time: String,
}

impl FastestLapSummaryEntry {
    fn parse(row: &ElementRef) -> Result<Self> {
        let helper = ScrapperHelper::new();

        let cols: Vec<_> = helper.table_cols(row).collect();
        if cols.len() != 4 {
            return Err(anyhow::anyhow!("invalid column count"));
        }

        let grand_prix = helper
            .inner_html(&cols[0])
            .with_context(|| "column: grand_prix")?;
        let driver = helper
            .join_spans(&cols[1])
            .with_context(|| "column: driver")?;
        let car = helper.inner_html(&cols[2]).with_context(|| "column: car")?;
        let time = helper
            .inner_html(&cols[3])
            .with_context(|| "column: time")?;

        Ok(Self {
            grand_prix,
            driver,
            car,
            time,
        })
    }
}
