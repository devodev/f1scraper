use std::fmt::Debug;

use anyhow::Context;
use scraper::{ElementRef, Html};

use crate::parse::HtmlTable;
use crate::prelude::*;

use super::ScrapperHelper;

#[derive(Default, Debug)]
pub struct RaceResult {
    pub year: u16,
    pub circuit: Circuit,
    pub data: Vec<RaceResultEntry>,
}

impl RaceResult {
    const TABLE_SELECTOR_STR: &str = "div.resultsarchive-wrapper>div.resultsarchive-content>div.resultsarchive-col-right>table.resultsarchive-table";

    pub fn parse(html: &str, year: u16, circuit: &Circuit) -> Result<Self> {
        // parse html
        let document = Html::parse_document(html);
        let document_root = document.root_element();

        // select table
        let table = HtmlTable::parse(&document_root, Self::TABLE_SELECTOR_STR)?;

        // parse rows
        let data: Result<Vec<_>, _> = table.rows().map(|r| RaceResultEntry::parse(&r)).collect();
        let data = data.with_context(|| "parse table rows")?;

        Ok(Self {
            year,
            circuit: circuit.clone(),
            data,
        })
    }
}

#[derive(Default, Debug)]
pub struct RaceResultEntry {
    pub pos: String,
    pub no: String,
    pub driver: String,
    pub car: String,
    pub laps: String,
    pub time_retired: String,
    pub pts: String,
}

impl RaceResultEntry {
    fn parse(row: &ElementRef) -> Result<Self> {
        let helper = ScrapperHelper::new();

        let cols: Vec<_> = helper.table_cols(row).collect();
        if cols.len() != 7 {
            return Err(anyhow::anyhow!("invalid column count"));
        }

        let pos = helper.inner_html(&cols[0]).with_context(|| "column: pos")?;
        let no = helper.inner_html(&cols[1]).with_context(|| "column: no")?;
        let driver = helper
            .join_spans(&cols[2])
            .with_context(|| "column: driver")?;
        let car = helper.inner_html(&cols[3]).with_context(|| "column: car")?;
        let laps = helper
            .inner_html(&cols[4])
            .with_context(|| "column: laps")?;
        let time_retired = helper
            .inner_html(&cols[5])
            .with_context(|| "column: time_retired")?;
        let pts = helper.inner_html(&cols[6]).with_context(|| "column:pts")?;

        Ok(Self {
            pos,
            no,
            driver,
            car,
            laps,
            time_retired,
            pts,
        })
    }
}

#[derive(Default, Debug)]
pub struct RaceSummary {
    pub year: u16,
    pub data: Vec<RaceSummaryEntry>,
}

impl RaceSummary {
    const TABLE_SELECTOR_STR: &str =
        "div.resultsarchive-content>div.table-wrap>table.resultsarchive-table";

    pub fn parse(html: &str, year: u16) -> Result<Self> {
        // parse html
        let document = Html::parse_document(html);
        let document_root = document.root_element();

        // select table
        let table = HtmlTable::parse(&document_root, Self::TABLE_SELECTOR_STR)?;

        // parse rows
        let data: Result<Vec<_>, _> = table.rows().map(|r| RaceSummaryEntry::parse(&r)).collect();
        let data = data.with_context(|| "parse table rows")?;

        Ok(Self { year, data })
    }
}

#[derive(Default, Debug)]
pub struct RaceSummaryEntry {
    pub grand_prix: String,
    pub url: String,
    pub date: String,
    pub winner: String,
    pub car: String,
    pub laps: String,
    pub time: String,
}

impl RaceSummaryEntry {
    fn parse(row: &ElementRef) -> Result<Self> {
        let helper = ScrapperHelper::new();

        let cols: Vec<_> = helper.table_cols(row).collect();
        if cols.len() != 6 {
            return Err(anyhow::anyhow!("invalid column count"));
        }

        let grand_prix = helper
            .link(&cols[0])
            .with_context(|| "column: grand prix")?;
        let grand_prix_col = helper
            .link_elem(&cols[0])
            .with_context(|| "column: grand prix")?;
        let url = helper
            .href(&grand_prix_col)
            .with_context(|| "column: grand prix")?;
        let date = helper
            .inner_html(&cols[1])
            .with_context(|| "column: date")?;
        let winner = helper
            .join_spans(&cols[2])
            .with_context(|| "column: winner")?;
        let car = helper.inner_html(&cols[3]).with_context(|| "column: car")?;
        let laps = helper
            .inner_html(&cols[4])
            .with_context(|| "column: laps")?;
        let time = helper
            .inner_html(&cols[5])
            .with_context(|| "column: time")?;

        Ok(Self {
            grand_prix,
            url,
            date,
            winner,
            car,
            laps,
            time,
        })
    }

    pub fn circuit(&self) -> Result<Circuit> {
        // Example:
        //   /en/results.html/1950/races/100/italy/race-result.html
        let tokens: Vec<_> = self.url.split('/').skip(5).take(2).collect();
        if tokens.len() != 2 {
            return Err(anyhow::anyhow!(
                "can't parse url: invalid format: {}",
                self.url
            ));
        }

        let idx_token = tokens[0];
        let idx = idx_token.parse::<u16>().with_context(|| {
            format!(
                "parse circuit index from url (token: `{}`): `{}`",
                idx_token, self.url
            )
        })?;
        let name = tokens[1];

        Ok(Circuit::new(idx, name, &self.grand_prix))
    }
}

#[derive(Default, Debug, Clone)]
pub struct Circuit {
    pub idx: u16,
    pub name: String,
    pub display_name: String,
}

impl Circuit {
    fn new<S: Into<String>>(idx: u16, name: S, display_name: S) -> Self {
        Self {
            idx,
            name: name.into(),
            display_name: display_name.into(),
        }
    }
}

#[derive(Default, Debug)]
pub struct FastestLap {
    pub pos: String,
    pub no: String,
    pub driver: String,
    pub car: String,
    pub lap: String,
    pub time: String,
}

#[derive(Default, Debug)]
pub struct Qualifying {
    pub pos: String,
    pub no: String,
    pub driver: String,
    pub car: String,
    pub time: String,
}
