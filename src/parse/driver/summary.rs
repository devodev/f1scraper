use anyhow::Context;
use scraper::element_ref::Select;
use scraper::{ElementRef, Html, Selector};
use selectors::attr::CaseSensitivity;

use crate::parse::{next_inner_html, HtmlTable};
use crate::prelude::*;
use crate::types::{DriverResultSummaryData, DriverResultSummaryHeaders, Table};

pub type DriverResultSummaryTable = Table<DriverResultSummaryHeaders, DriverResultSummaryData>;

const TABLE_SELECTOR_STR: &str =
    "div.resultsarchive-wrapper>div.resultsarchive-content>div.table-wrap>table.resultsarchive-table";

pub fn parse(html: &str, year: u16) -> Result<DriverResultSummaryTable> {
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

fn parse_headers(s: Select) -> Result<DriverResultSummaryHeaders> {
    let headers: Vec<String> = s
        .filter(|col| {
            !col.value()
                .has_class("limiter", CaseSensitivity::AsciiCaseInsensitive)
        })
        .map(|x| x.inner_html())
        .collect();
    let header_count = headers.len();
    if header_count != 5 {
        return Err(anyhow::anyhow!("invalid header count: {header_count}"));
    }

    Ok(DriverResultSummaryHeaders {
        pos: headers[0].to_owned(),
        driver: headers[1].to_owned(),
        nationality: headers[2].to_owned(),
        car: headers[3].to_owned(),
        pts: headers[4].to_owned(),
    })
}

fn parse_row(row: &ElementRef) -> Result<DriverResultSummaryData> {
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

    let driver = cols
        .next()
        .ok_or(anyhow::anyhow!("expected column: driver"))?
        .select(&span)
        .map(|x| x.inner_html())
        .collect::<Vec<String>>()
        .join(" ")
        .trim()
        .to_string();
    let nationality = next_inner_html(&mut cols).with_context(|| "column: nationality")?;
    let car = next_inner_html(&mut cols).with_context(|| "column: car")?;
    let pts = next_inner_html(&mut cols).with_context(|| "column: pts")?;

    Ok(DriverResultSummaryData {
        pos,
        url,
        driver,
        nationality,
        car,
        pts,
    })
}
