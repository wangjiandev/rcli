use csv::Reader;
use serde_json::Value;
use std::fs::File;

use crate::cli::csv_options::OutputFormat;

pub fn process_csv(input: &str, output: &str, format: &OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);

    let headers = reader.headers()?.clone();
    for record in reader.records() {
        let record = record?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }

    match format {
        OutputFormat::Json => {
            let file = File::create(output)?;
            serde_json::to_writer_pretty(file, &ret)?;
        }
        OutputFormat::Yaml => {
            let file = File::create(output)?;
            serde_yaml::to_writer(file, &ret)?;
        }
    }

    Ok(())
}
