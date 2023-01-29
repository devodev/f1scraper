use crate::prelude::*;

use super::ScrapeContext;

mod summary;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    /// Scrape race summaries
    Summary(summary::Args),
}

pub fn process(scrape_ctx: ScrapeContext, cmd: Commands) -> Result<()> {
    match cmd {
        Commands::Summary(args) => summary::run(scrape_ctx, args),
    }
}
