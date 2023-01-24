mod result;
mod summary;

pub use summary::parse as parse_summary;
pub use summary::RaceResultSummaryTable;

pub use result::parse as parse_result;
pub use result::RaceResultTable;