use anyhow::Context;
use scraper::{ElementRef, Html, Selector};
use selectors::attr::CaseSensitivity;

use crate::parse::{next_inner_html, HtmlTable};
use crate::prelude::*;
use crate::types::TeamSummary;
use crate::types::TeamSummaryEntry;

const TABLE_SELECTOR_STR: &str =
    "div.resultsarchive-content>div.table-wrap>table.resultsarchive-table";

pub fn parse(html: &str, year: u16) -> Result<TeamSummary> {
    // parse html
    let document = Html::parse_document(html);
    let document_root = document.root_element();

    // select table
    let table = HtmlTable::parse(&document_root, TABLE_SELECTOR_STR)?;

    // parse rows
    let data: Result<Vec<_>, _> = table.rows().map(|r| parse_row(&r)).collect();
    let data = data.with_context(|| "parse table rows")?;

    Ok(TeamSummary { year, data })
}

fn parse_row(row: &ElementRef) -> Result<TeamSummaryEntry> {
    let a = Selector::parse("a").unwrap();
    let td = Selector::parse("td").unwrap();

    let mut cols = row.select(&td).filter(|row| {
        !row.value()
            .has_class("limiter", CaseSensitivity::AsciiCaseInsensitive)
    });

    let pos = cols
        .next()
        .ok_or(anyhow::anyhow!("expected table column: pos"))?
        .inner_html()
        .trim()
        .to_string();
    let team_col = cols
        .next()
        .ok_or(anyhow::anyhow!("expected table column: team"))?
        .select(&a)
        .next()
        .ok_or(anyhow::anyhow!("expected a element on table column: team"))?;
    let url = team_col
        .value()
        .attr("href")
        .ok_or(anyhow::anyhow!(
            "expected a element to contain url on column: team"
        ))?
        .trim()
        .to_string();
    let team = team_col.inner_html().trim().to_string();
    let pts = next_inner_html(&mut cols).with_context(|| "column: pts")?;

    Ok(TeamSummaryEntry {
        pos,
        url,
        team,
        pts,
    })
}
