use std::fmt::Debug;

use anyhow::Context;
use scraper::{ElementRef, Html, Selector};
use selectors::attr::CaseSensitivity;

use crate::parse::{next_inner_html, HtmlTable};
use crate::prelude::*;

#[derive(Default, Debug)]
pub struct TeamResult {
    pub year: u16,
    pub team: Team,
    pub data: Vec<TeamResultEntry>,
}

impl TeamResult {
    const TABLE_SELECTOR_STR: &str = "div.resultsarchive-wrapper>div.resultsarchive-content>div.table-wrap>table.resultsarchive-table";

    pub fn parse(html: &str, year: u16, team: &Team) -> Result<Self> {
        // parse html
        let document = Html::parse_document(html);
        let document_root = document.root_element();

        // select table
        let table = HtmlTable::parse(&document_root, Self::TABLE_SELECTOR_STR)?;

        // parse rows
        let data: Result<Vec<_>, _> = table.rows().map(|r| TeamResultEntry::parse(&r)).collect();
        let data = data.with_context(|| "parse table rows")?;

        Ok(Self {
            year,
            team: team.clone(),
            data,
        })
    }
}

#[derive(Default, Debug)]
pub struct TeamResultEntry {
    pub grand_prix: String,
    pub date: String,
    pub pts: String,
}

impl TeamResultEntry {
    fn parse(row: &ElementRef) -> Result<Self> {
        let a = Selector::parse("a").unwrap();
        let td = Selector::parse("td").unwrap();

        let mut cols = row.select(&td).filter(|row| {
            !row.value()
                .has_class("limiter", CaseSensitivity::AsciiCaseInsensitive)
        });

        let grand_prix = cols
            .next()
            .ok_or(anyhow::anyhow!("expected column: grand prix"))?
            .select(&a)
            .map(|x| x.inner_html())
            .next()
            .ok_or(anyhow::anyhow!(
                "expected column: grand prix in <a> element"
            ))?
            .trim()
            .to_string();
        let date = next_inner_html(&mut cols).with_context(|| "column: date")?;
        let pts = next_inner_html(&mut cols).with_context(|| "column: pts")?;

        Ok(Self {
            grand_prix,
            date,
            pts,
        })
    }
}

#[derive(Default, Debug)]
pub struct TeamSummary {
    pub year: u16,
    pub data: Vec<TeamSummaryEntry>,
}

impl TeamSummary {
    const TABLE_SELECTOR_STR: &str =
        "div.resultsarchive-content>div.table-wrap>table.resultsarchive-table";

    pub fn parse(html: &str, year: u16) -> Result<TeamSummary> {
        // parse html
        let document = Html::parse_document(html);
        let document_root = document.root_element();

        // select table
        let table = HtmlTable::parse(&document_root, Self::TABLE_SELECTOR_STR)?;

        // parse rows
        let data: Result<Vec<_>, _> = table.rows().map(|r| TeamSummaryEntry::parse(&r)).collect();
        let data = data.with_context(|| "parse table rows")?;

        Ok(TeamSummary { year, data })
    }
}

#[derive(Default, Debug)]
pub struct TeamSummaryEntry {
    pub pos: String,
    pub url: String,
    pub team: String,
    pub pts: String,
}

impl TeamSummaryEntry {
    fn parse(row: &ElementRef) -> Result<Self> {
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

        Ok(Self {
            pos,
            url,
            team,
            pts,
        })
    }
}

impl TeamSummaryEntry {
    pub fn team(&self) -> Result<Team> {
        // Example:
        //   /en/results.html/1950/team/alfa_romeo_ferrari.html
        let tokens: Vec<_> = self.url.split('/').skip(5).take(1).collect();
        if tokens.len() != 1 {
            return Err(anyhow::anyhow!(
                "can't parse url: invalid format: {}",
                self.url
            ));
        }

        let name = tokens[0].trim_end_matches(".html");

        Ok(Team::new(name, &self.team))
    }
}

#[derive(Default, Debug, Clone)]
pub struct Team {
    pub name: String,
    pub display_name: String,
}

impl Team {
    fn new<S: Into<String>>(name: S, display_name: S) -> Self {
        Self {
            name: name.into(),
            display_name: display_name.into(),
        }
    }
}
