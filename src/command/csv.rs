use clap::Args;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Args, Debug)]
pub struct CsvArgs {
    #[arg(short, long, value_parser=verify_file)]
    pub input: String,
    #[arg(short, long, default_value = "./assets/juventus.json")]
    pub output: String,
    #[arg(short, long, default_value = "json")]
    pub format: Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Format {
    Json,
    Yaml,
}

impl From<Format> for &'static str {
    fn from(format: Format) -> &'static str {
        match format {
            Format::Json => "json",
            Format::Yaml => "yaml",
        }
    }
}

impl std::str::FromStr for Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Format::Json),
            "yaml" => Ok(Format::Yaml),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

pub fn verify_file(path: &str) -> anyhow::Result<String> {
    if std::path::Path::new(path).exists() {
        Ok(path.to_string())
    } else {
        Err(anyhow::anyhow!("File not found"))
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    pub name: String,
    pub position: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub kit_number: u8,
}

pub fn read_csv(path: &str) -> anyhow::Result<Vec<Player>> {
    let mut rdr = Reader::from_path(path)?;
    let mut players = Vec::new();
    for result in rdr.deserialize() {
        let player: Player = result?;
        players.push(player);
    }
    Ok(players)
}

pub fn write_json(path: &str, players: Vec<Player>) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(&players)?;
    std::fs::write(path, json)?;
    Ok(())
}

pub fn process_csv(arg: &CsvArgs) -> anyhow::Result<()> {
    let input = arg.input.clone();
    let output = arg.output.clone();
    let format = arg.format;
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(40);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        let value_iter = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        ret.push(value_iter);
    }

    let contents = match format {
        Format::Json => serde_json::to_string_pretty(&ret)?,
        Format::Yaml => serde_yaml::to_string(&ret)?,
    };
    std::fs::write(output, contents)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_csv() {
        let players = read_csv("./assets/juventus.csv").unwrap();
        assert_eq!(players.len(), 27);
    }
}
