use anyhow::Context;
use scraper::element_ref::Select;
use scraper::{ElementRef, Html, Selector};
use selectors::attr::CaseSensitivity;

use crate::parse::{next_inner_html, HtmlTable};
use crate::prelude::*;
use crate::types::{RaceResultSummaryData, RaceResultSummaryHeaders, Table};

pub type RaceResultSummaryTable = Table<RaceResultSummaryHeaders, RaceResultSummaryData>;

const TABLE_SELECTOR_STR: &str =
    "div.resultsarchive-content>div.table-wrap>table.resultsarchive-table";

pub fn parse(html: &str, year: u16) -> Result<RaceResultSummaryTable> {
    // parse html
    let document = Html::parse_document(html);
    let document_root = document.root_element();

    // select table
    let table = HtmlTable::parse(&document_root, TABLE_SELECTOR_STR)?;

    // parse headers from table
    let headers = parse_headers(table.headers()).with_context(|| "parse table headers")?;

    // parse content
    let data: Result<Vec<_>, _> = table.rows().map(|r| parse_row(&r)).collect();
    let data = data.with_context(|| "parse table rows")?;

    Ok(Table::new(year, headers, data))
}

fn parse_headers(s: Select) -> Result<RaceResultSummaryHeaders> {
    let headers: Vec<String> = s
        .filter(|col| {
            !col.value()
                .has_class("limiter", CaseSensitivity::AsciiCaseInsensitive)
        })
        .map(|x| x.inner_html())
        .collect();
    let header_count = headers.len();
    if header_count != 6 {
        return Err(anyhow::anyhow!("invalid header count: {header_count}"));
    }

    Ok(RaceResultSummaryHeaders {
        grand_prix: headers[0].to_owned(),
        date: headers[1].to_owned(),
        winner: headers[2].to_owned(),
        car: headers[3].to_owned(),
        laps: headers[4].to_owned(),
        time: headers[5].to_owned(),
    })
}

fn parse_row(row: &ElementRef) -> Result<RaceResultSummaryData> {
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

    Ok(RaceResultSummaryData {
        grand_prix,
        url,
        date,
        winner,
        car,
        laps,
        time,
    })
}
