use crate::prelude::*;

use super::ScrapeContext;

mod result;
mod summary;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    /// Scrape driver standings summaries
    Summary(summary::Args),

    /// Scrape driver standings results
    Result(result::Args),
}

pub fn run(scrape_ctx: ScrapeContext, cmd: Commands) -> Result<()> {
    match cmd {
        Commands::Summary(args) => summary::run(scrape_ctx, args),
        Commands::Result(args) => result::run(scrape_ctx, args),
    }
}
