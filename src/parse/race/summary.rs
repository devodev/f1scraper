use anyhow::Context;
use scraper::{ElementRef, Html, Selector};
use selectors::attr::CaseSensitivity;

use crate::format::{RaceResultSummaryData, RaceResultSummaryHeaders, Table};
use crate::parse::inner_html_to_string;
use crate::prelude::*;

pub type RaceResultSummaryTable = Table<RaceResultSummaryHeaders, RaceResultSummaryData>;

pub fn parse(html: &str, year: u16) -> Result<RaceResultSummaryTable> {
    // parse html
    let document = Html::parse_document(&html);

    // select table
    let table_selector =
        Selector::parse("div.resultsarchive-content>div.table-wrap>table.resultsarchive-table")
            .unwrap();
    let table = document
        .select(&table_selector)
        .next()
        .with_context(|| "selecting table")?;

    // parse headers from table
    let headers = parse_headers(&table)?;

    // parse summaries from table
    let rows_selector = Selector::parse("tbody>tr").unwrap();
    let rows = table.select(&rows_selector);

    let data: Result<Vec<_>, _> = rows.map(|r| parse_data(&r)).collect();
    let data = data?;

    Ok(Table::new(year, headers, data))
}

fn parse_headers(table: &ElementRef) -> Result<RaceResultSummaryHeaders> {
    let headers_selector = Selector::parse("thead>tr>th").unwrap();
    let headers: Vec<String> = table
        .select(&headers_selector)
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

fn parse_data(row: &ElementRef) -> Result<RaceResultSummaryData> {
    let a = Selector::parse("a").unwrap();
    let td = Selector::parse("td").unwrap();
    let span = Selector::parse("span").unwrap();

    let mut cols = row
        .select(&td)
        .filter(|row| {
            !row.value()
                .has_class("limiter", CaseSensitivity::AsciiCaseInsensitive)
        })
        .into_iter();

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
    let date = inner_html_to_string(&mut cols).with_context(|| "column: date")?;
    let winner = cols
        .next()
        .ok_or(anyhow::anyhow!("expected column: winner"))?
        .select(&span)
        .map(|x| x.inner_html())
        .collect::<Vec<String>>()
        .join(" ")
        .trim()
        .to_string();
    let car = inner_html_to_string(&mut cols).with_context(|| "column: car")?;
    let laps = inner_html_to_string(&mut cols).with_context(|| "column: laps")?;
    let time = inner_html_to_string(&mut cols).with_context(|| "column: time")?;

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
