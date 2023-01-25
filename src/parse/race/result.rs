use anyhow::Context;
use scraper::{ElementRef, Html, Selector};
use selectors::attr::CaseSensitivity;

use crate::parse::{next_inner_html, HtmlTable};
use crate::prelude::*;
use crate::types::{Circuit, RaceResult};

const TABLE_SELECTOR_STR: &str = "div.resultsarchive-wrapper>div.resultsarchive-content>div.resultsarchive-col-right>table.resultsarchive-table";

pub struct ParsedRaceResult {
    pub year: u16,
    pub circuit: Circuit,
    pub data: Vec<RaceResult>,
}

pub fn parse(html: &str, year: u16, circuit: &Circuit) -> Result<ParsedRaceResult> {
    // parse html
    let document = Html::parse_document(html);
    let document_root = document.root_element();

    // select table
    let table = HtmlTable::parse(&document_root, TABLE_SELECTOR_STR)?;

    // parse rows
    let data: Result<Vec<_>, _> = table.rows().map(|r| parse_row(&r)).collect();
    let data = data.with_context(|| "parse table rows")?;

    Ok(ParsedRaceResult {
        year,
        circuit: circuit.clone(),
        data,
    })
}

fn parse_row(row: &ElementRef) -> Result<RaceResult> {
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

    Ok(RaceResult {
        pos,
        no,
        driver,
        car,
        laps,
        time_retired,
        pts,
    })
}
