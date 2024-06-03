use anyhow::Ok;
use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

use crate::opts::OutputFormat;

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

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut records = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        // headers.iter() -> Name, Position, DOB, National
        // record.iter() -> 使用record的迭代器
        // zip -> 将两个迭代器合并成一个元组[(header, record),...]
        // collect::<Value>() -> 将合并后的元组转换成json格式
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        records.push(json_value);
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&records)?,
        OutputFormat::Yaml => serde_yaml::to_string(&records)?,
    };
    fs::write(output, content)?;
    Ok(())
}
