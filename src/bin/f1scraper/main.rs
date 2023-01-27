use std::io::Write;

use anyhow::{Context, Result};
use chrono::Local;
use clap::Parser;
use env_logger::Builder;

mod commands;

mod prelude {
    pub use anyhow::{Context, Result};
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: commands::Commands,

    /// Enable info(-v), debug(-vv) or trace(-vvv) logging
    #[arg(long, short = 'v', action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() -> Result<()> {
    // Parse cli
    let cli = Cli::parse();

    // Setup logger
    let mut logger_builder = Builder::from_default_env();
    logger_builder.format(|buf, record| {
        write!(
            buf,
            "[{}]",
            Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
        )?;
        write!(buf, "[{}]", record.level())?;
        if let Some(path) = record.module_path() {
            write!(buf, "[{path}]")?;
        };
        writeln!(buf, " {}", record.args())?;
        Ok(())
    });
    if cli.verbose == 1 {
        logger_builder.filter_level(log::LevelFilter::Info);
    }
    if cli.verbose == 2 {
        logger_builder.filter_level(log::LevelFilter::Debug);
    }
    if cli.verbose > 2 {
        logger_builder.filter_level(log::LevelFilter::Trace);
    }
    logger_builder.init();

    // Run command
    let cmd_name = cli.command.to_string();
    commands::process(cli.command).with_context(|| format!("process command `{cmd_name}`"))
}
