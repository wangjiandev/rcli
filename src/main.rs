use std::path::Path;

use clap::{command, Args, Parser, Subcommand};
use csv::Reader;
use serde::{Deserialize, Serialize};

/// rcli csv -i input.csv -o output.json --header -d ','

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Parser, Debug)]
#[command(name="rcli", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Show CSV or Convert CSV to JSON tools
    #[command(name = "csv")]
    Csv(CsvOptions),
}

#[derive(Args, Debug)]
struct CsvOptions {
    /// Input CSV file path
    #[arg(short, long, value_parser = verify_input_file)]
    input: String,
    /// Output JSON file path
    #[arg(short, long, default_value = "output.json")]
    output: String,
    /// Show header
    #[arg(long, default_value_t = true)]
    header: bool,
    /// Delimiter
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,
}

fn verify_input_file(path: &str) -> Result<String, String> {
    if !Path::new(path).exists() {
        return Err(format!("Input file {} not found", path));
    }
    Ok(path.into())
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Csv(options) => {
            let mut reader = Reader::from_path(options.input).unwrap();
            let players = reader
                .deserialize()
                .map(|record| record.unwrap())
                .collect::<Vec<Player>>();
            println!("{:?}", players);
        }
    }
}
