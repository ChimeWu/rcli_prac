use clap::Args;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Args, Debug)]
pub struct CsvArgs {
    #[arg(short, long, default_value = "./assets/juventus.csv")]
    pub input: String,
    #[arg(short, long, default_value = "./assets/juventus.json")]
    pub output: String,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_csv() {
        let players = read_csv("./assets/juventus.csv").unwrap();
        assert_eq!(players.len(), 27);
    }
}
