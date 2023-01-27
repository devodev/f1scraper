use std::fmt::Debug;

use anyhow::Context;
use scraper::{ElementRef, Html, Selector};
use selectors::attr::CaseSensitivity;

use crate::parse::{next_inner_html, HtmlTable};
use crate::prelude::*;

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
        let td = Selector::parse("td").unwrap();
        let span = Selector::parse("span").unwrap();

        let mut cols = row.select(&td).filter(|row| {
            !row.value()
                .has_class("limiter", CaseSensitivity::AsciiCaseInsensitive)
        });

        let pos = next_inner_html(&mut cols).with_context(|| "column: pos")?;
        let no = next_inner_html(&mut cols).with_context(|| "column: no")?;
        let driver = cols
            .next()
            .ok_or(anyhow::anyhow!("expected column: driver"))?
            .select(&span)
            .map(|x| x.inner_html())
            .collect::<Vec<String>>()
            .join(" ")
            .trim()
            .to_string();
        let car = next_inner_html(&mut cols).with_context(|| "column: car")?;
        let laps = next_inner_html(&mut cols).with_context(|| "column: laps")?;
        let time_retired = next_inner_html(&mut cols).with_context(|| "column: time_retired")?;
        let pts = next_inner_html(&mut cols).with_context(|| "column:pts")?;

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
        let a = Selector::parse("a").unwrap();
        let td = Selector::parse("td").unwrap();
        let span = Selector::parse("span").unwrap();

        let mut cols = row.select(&td).filter(|row| {
            !row.value()
                .has_class("limiter", CaseSensitivity::AsciiCaseInsensitive)
        });

        let grand_prix_col = cols
            .next()
            .ok_or(anyhow::anyhow!("expected table column: grand prix"))?
            .select(&a)
            .next()
            .ok_or(anyhow::anyhow!(
                "expected a element on table column: grand prix"
            ))?;

        let url = grand_prix_col
            .value()
            .attr("href")
            .ok_or(anyhow::anyhow!(
                "expected a element to contain url on column: grand prix"
            ))?
            .trim()
            .to_string();

        let grand_prix = grand_prix_col.inner_html().trim().to_string();
        let date = next_inner_html(&mut cols).with_context(|| "column: date")?;
        let winner = cols
            .next()
            .ok_or(anyhow::anyhow!("expected column: winner"))?
            .select(&span)
            .map(|x| x.inner_html())
            .collect::<Vec<String>>()
            .join(" ")
            .trim()
            .to_string();
        let car = next_inner_html(&mut cols).with_context(|| "column: car")?;
        let laps = next_inner_html(&mut cols).with_context(|| "column: laps")?;
        let time = next_inner_html(&mut cols).with_context(|| "column: time")?;

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
