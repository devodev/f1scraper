pub mod parse;
pub mod scrape;
pub mod types;

mod prelude {
    pub use anyhow::{Context, Result};
    pub use log::{debug, info};
}
