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
    /// Scrape Race Summaries
    Summary(summary::Args),

    /// Scrape Race Result
    Result(result::Args),
}

pub fn process(scrape_ctx: ScrapeContext, cmd: Commands) -> Result<()> {
    match cmd {
        Commands::Summary(args) => summary::process(scrape_ctx, args),
        Commands::Result(args) => result::process(scrape_ctx, args),
    }
}
