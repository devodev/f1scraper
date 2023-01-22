use crate::prelude::*;

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
}

pub fn process(cmd: Commands) -> Result<()> {
    match cmd {
        Commands::Summary(args) => summary::process(args),
    }
}
