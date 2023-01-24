use anyhow::Context;
use scraper::element_ref::Select;
use scraper::{ElementRef, Html, Selector};
use selectors::attr::CaseSensitivity;

use crate::parse::{inner_html_to_string, HtmlTable};
use crate::prelude::*;
use crate::types::{Circuit, RaceResultData, RaceResultHeaders, Table};

pub type RaceResultTable = Table<RaceResultHeaders, RaceResultData>;

const TABLE_SELECTOR_STR: &str = "div.resultsarchive-wrapper>div.resultsarchive-content>div.resultsarchive-col-right>table.resultsarchive-table";

pub fn parse(html: &str, year: u16, circuit: &Circuit) -> Result<RaceResultTable> {
    // parse html
    let document = Html::parse_document(html);
    let document_root = document.root_element();

    // select table
    let table = HtmlTable::parse(&document_root, TABLE_SELECTOR_STR)?;

    // parse headers from table
    let headers = parse_headers(table.headers())?;

    // parse content
    let content: Result<Vec<_>, _> = table.rows().map(|r| parse_row(&r)).collect();
    let content = content?;

    Ok(Table::new(year, headers, content).with_circuit(circuit.clone()))
}

fn parse_headers(s: Select) -> Result<RaceResultHeaders> {
    let headers: Vec<String> = s
        .filter(|col| {
            !col.value()
                .has_class("limiter", CaseSensitivity::AsciiCaseInsensitive)
        })
        .map(|x| {
            if let Some(child) = x.first_child() {
                ElementRef::wrap(child).unwrap_or(x)
            } else {
                x
            }
        })
        .map(|x| x.inner_html())
        .collect();
    let header_count = headers.len();
    if header_count != 7 {
        return Err(anyhow::anyhow!("invalid header count: {header_count}"));
    }

    Ok(RaceResultHeaders {
        pos: headers[0].to_owned(),
        no: headers[1].to_owned(),
        driver: headers[2].to_owned(),
        car: headers[3].to_owned(),
        laps: headers[4].to_owned(),
        time_retired: headers[5].to_owned(),
        pts: headers[6].to_owned(),
    })
}

fn parse_row(row: &ElementRef) -> Result<RaceResultData> {
    let td = Selector::parse("td").unwrap();
    let span = Selector::parse("span").unwrap();

    let mut cols = row.select(&td).filter(|row| {
        !row.value()
            .has_class("limiter", CaseSensitivity::AsciiCaseInsensitive)
    });

    let pos = inner_html_to_string(&mut cols).with_context(|| "column: pos")?;
    let no = inner_html_to_string(&mut cols).with_context(|| "column: no")?;
    let driver = cols
        .next()
        .ok_or(anyhow::anyhow!("expected column: driver"))?
        .select(&span)
        .map(|x| x.inner_html())
        .collect::<Vec<String>>()
        .join(" ")
        .trim()
        .to_string();
    let car = inner_html_to_string(&mut cols).with_context(|| "column: car")?;
    let laps = inner_html_to_string(&mut cols).with_context(|| "column: laps")?;
    let time_retired = inner_html_to_string(&mut cols).with_context(|| "column: time_retired")?;
    let pts = inner_html_to_string(&mut cols).with_context(|| "column:pts")?;

    Ok(RaceResultData {
        pos,
        no,
        driver,
        car,
        laps,
        time_retired,
        pts,
    })
}
