use anyhow::Result;
use clap::Parser;
use std::{path::Path, str::FromStr};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "CSV file operations")]
    Csv(CsvOpts),
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Toml => "toml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            _ => Err(anyhow::anyhow!("Invalid format: {}", s)),
        }
    }
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")] // "output.json".into()
    pub output: String,
    #[arg(short, long, default_value_t = ',')]
    pub deliniter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
    #[arg(short, long, value_parser = parser_format, default_value = "json")]
    pub format: OutputFormat,
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err(format!("File not found: {}", filename))
    }
}

fn parser_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}
