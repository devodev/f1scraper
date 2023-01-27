use scraper::ElementRef;
use scraper::Selector;

use crate::prelude::*;

mod driver;
mod race;
mod team;

pub use race::Circuit;
pub use race::RaceResult;
pub use race::RaceResultEntry;
pub use race::RaceSummary;
pub use race::RaceSummaryEntry;

pub use driver::DriverFragment;
pub use driver::DriverResult;
pub use driver::DriverResultEntry;
pub use driver::DriverSummary;
pub use driver::DriverSummaryEntry;

use selectors::attr::CaseSensitivity;
pub use team::Team;
pub use team::TeamResult;
pub use team::TeamResultEntry;
pub use team::TeamSummary;
pub use team::TeamSummaryEntry;

// use log::info;
// info!(
//     "\n=============================== inner_html \n{}\n=============================== inner_html",
//     cols[0].inner_html()
// );

#[derive(Debug)]
struct ScrapperHelper {
    selector_a: Selector,
    selector_td: Selector,
    selector_span: Selector,
}

impl ScrapperHelper {
    fn new() -> Self {
        Self {
            selector_a: Selector::parse("a").unwrap(),
            selector_td: Selector::parse("td").unwrap(),
            selector_span: Selector::parse("span").unwrap(),
        }
    }

    fn inner_html(&self, elem: &ElementRef) -> Result<String> {
        let s = elem.inner_html().trim().to_string();
        Ok(s)
    }

    fn link_elem<'a>(&'a self, elem: &'a ElementRef) -> Result<ElementRef> {
        let s = elem
            .select(&self.selector_a)
            .next()
            .ok_or(anyhow::anyhow!("no <a> element found"))?;
        Ok(s)
    }

    fn link(&self, elem: &ElementRef) -> Result<String> {
        let s = elem
            .select(&self.selector_a)
            .map(|x| x.inner_html())
            .next()
            .ok_or(anyhow::anyhow!("no <a> element found"))?
            .trim()
            .to_string();
        Ok(s)
    }

    fn join_spans(&self, elem: &ElementRef) -> Result<String> {
        let s = elem
            .select(&self.selector_span)
            .map(|x| x.inner_html())
            .collect::<Vec<String>>()
            .join(" ")
            .trim()
            .to_string();
        Ok(s)
    }

    fn table_cols<'a>(&'a self, elem: &'a ElementRef) -> impl Iterator<Item = ElementRef<'a>> {
        elem.select(&self.selector_td).filter(|row| {
            !row.value()
                .has_class("limiter", CaseSensitivity::AsciiCaseInsensitive)
        })
    }

    fn href(&self, elem: &ElementRef) -> Result<String> {
        let s = elem
            .value()
            .attr("href")
            .ok_or(anyhow::anyhow!("no href attribute found"))?
            .trim()
            .to_string();
        Ok(s)
    }
}
