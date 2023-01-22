use scraper::ElementRef;

use crate::prelude::*;

mod races;

pub use races::parse_races;
pub use races::parse_races_summary;
pub use races::RaceResultSummaryTable;
pub use races::RaceResultTable;

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
