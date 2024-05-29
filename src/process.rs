use std::fs;

use anyhow::Ok;
use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};

// Name,Position,DOB,Nationality,Kit Number
#[derive(Debug, Deserialize, Serialize)]
struct Player {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let records = reader
        .deserialize()
        .map(|record| record.unwrap())
        .collect::<Vec<Player>>();
    let json = serde_json::to_string_pretty(&records)?;
    fs::write(output, json)?;
    Ok(())
}
