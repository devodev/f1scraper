use scraper::ElementRef;

use crate::prelude::*;

mod race;

pub use race::parse_result;
pub use race::parse_summary;
pub use race::RaceResultSummaryTable;
pub use race::RaceResultTable;

pub(crate) fn inner_html_to_string<'a>(
    i: &mut impl Iterator<Item = ElementRef<'a>>,
) -> Result<String> {
    let s = i
        .next()
        .ok_or(anyhow::anyhow!("expected inner html"))?
        .inner_html()
        .trim()
        .to_string();
    Ok(s)
}
