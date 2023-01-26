use anyhow::Context;
use scraper::{ElementRef, Html, Selector};
use selectors::attr::CaseSensitivity;

use crate::parse::{next_inner_html, HtmlTable};
use crate::prelude::*;
use crate::types::{DriverSummary, DriverSummaryEntry};

const TABLE_SELECTOR_STR: &str =
    "div.resultsarchive-wrapper>div.resultsarchive-content>div.table-wrap>table.resultsarchive-table";

pub fn parse(html: &str, year: u16) -> Result<DriverSummary> {
    // parse html
    let document = Html::parse_document(html);
    let document_root = document.root_element();

    // select table
    let table = HtmlTable::parse(&document_root, TABLE_SELECTOR_STR)?;

    // parse rows
    let data: Result<Vec<_>, _> = table.rows().map(|r| parse_row(&r)).collect();
    let data = data.with_context(|| "parse table rows")?;

    Ok(DriverSummary { year, data })
}

fn parse_row(row: &ElementRef) -> Result<DriverSummaryEntry> {
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

    Ok(DriverSummaryEntry {
        pos,
        url,
        driver,
        nationality,
        car,
        pts,
    })
}
