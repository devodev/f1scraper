use anyhow::Context;
use scraper::element_ref::Select;
use scraper::{ElementRef, Html, Selector};
use selectors::attr::CaseSensitivity;

use crate::parse::{next_inner_html, HtmlTable};
use crate::prelude::*;
use crate::types::{DriverFragment, DriverResultData, DriverResultHeaders, Table};

pub type DriverResultTable = Table<DriverResultHeaders, DriverResultData>;

const TABLE_SELECTOR_STR: &str =
    "div.resultsarchive-wrapper>div.resultsarchive-content>div.table-wrap>table.resultsarchive-table";

pub fn parse(html: &str, year: u16, driver: &DriverFragment) -> Result<DriverResultTable> {
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

    Ok(Table::new(year, headers, data).with_driver(driver.clone()))
}

fn parse_headers(s: Select) -> Result<DriverResultHeaders> {
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
    if header_count != 5 {
        return Err(anyhow::anyhow!("invalid header count: {header_count}"));
    }

    Ok(DriverResultHeaders {
        grand_prix: headers[0].to_owned(),
        date: headers[1].to_owned(),
        pos: headers[2].to_owned(),
        car: headers[3].to_owned(),
        pts: headers[4].to_owned(),
    })
}

fn parse_row(row: &ElementRef) -> Result<DriverResultData> {
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

    Ok(DriverResultData {
        grand_prix,
        date,
        pos,
        car,
        pts,
    })
}
