use clap::Parser;
use command::Commands;

pub mod command;
pub mod progress;

#[derive(Parser, Debug)]
pub struct MyCli {
    #[command(subcommand)]
    subcmd: Commands,
}

pub fn run() -> anyhow::Result<()> {
    let args = MyCli::parse();
    match args.subcmd {
        Commands::Csv(args) => {
            command::csv::process_csv(&args)?;
        }
    }
    Ok(())
}
