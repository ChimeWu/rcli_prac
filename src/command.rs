use self::csv::CsvArgs;
use clap::Subcommand;

pub mod csv;

#[derive(Subcommand, Debug)]
pub enum Commands {
    Csv(CsvArgs),
}
