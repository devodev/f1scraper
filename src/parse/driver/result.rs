use anyhow::Context;
use scraper::{ElementRef, Html, Selector};
use selectors::attr::CaseSensitivity;

use crate::parse::{next_inner_html, HtmlTable};
use crate::prelude::*;
use crate::types::{DriverFragment, DriverResult};

const TABLE_SELECTOR_STR: &str =
    "div.resultsarchive-wrapper>div.resultsarchive-content>div.table-wrap>table.resultsarchive-table";

pub struct ParsedDriverResult {
    pub year: u16,
    pub driver: DriverFragment,
    pub data: Vec<DriverResult>,
}

pub fn parse(html: &str, year: u16, driver: &DriverFragment) -> Result<ParsedDriverResult> {
    // parse html
    let document = Html::parse_document(html);
    let document_root = document.root_element();

    // select table
    let table = HtmlTable::parse(&document_root, TABLE_SELECTOR_STR)?;

    // parse content
    let data: Result<Vec<_>, _> = table.rows().map(|r| parse_row(&r)).collect();
    let data = data.with_context(|| "parse table rows")?;

    Ok(ParsedDriverResult {
        year,
        driver: driver.clone(),
        data,
    })
}

fn parse_row(row: &ElementRef) -> Result<DriverResult> {
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

    Ok(DriverResult {
        grand_prix,
        date,
        pos,
        car,
        pts,
    })
}
