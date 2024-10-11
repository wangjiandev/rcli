use clap::{Args, Parser, Subcommand};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(name="rcli", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Show CSV or Convert CSV to JSON tools
    #[command(name = "csv")]
    Csv(CsvOptions),

    /// Generate Password
    #[command(name = "gen-pass")]
    GenPass(GenPassOptions),
}

#[derive(Args, Debug)]
pub struct CsvOptions {
    /// Input CSV file path
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    /// Output JSON file path
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    /// Show header
    #[arg(long, default_value_t = true)]
    pub header: bool,
    /// Delimiter
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    /// Output format
    #[arg(short, long, value_parser = parse_output_format, default_value = "json")]
    pub format: OutputFormat,
}

#[derive(Args, Debug)]
pub struct GenPassOptions {
    /// Password length
    #[arg(short, long, default_value_t = 16)]
    pub length: usize,
    /// Include uppercase letters
    #[arg(long, default_value_t = false)]
    pub no_uppercase: bool,
    /// Include lowercase letters
    #[arg(long, default_value_t = false)]
    pub no_lowercase: bool,
    /// Include numbers
    #[arg(long, default_value_t = false)]
    pub no_numbers: bool,
    /// Include symbols
    #[arg(long, default_value_t = false)]
    pub no_symbols: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

fn verify_input_file(path: &str) -> Result<String, anyhow::Error> {
    if !Path::new(path).exists() {
        return Err(anyhow::format_err!("Input file {} not found", path));
    }
    Ok(path.into())
}

fn parse_output_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    match format.to_lowercase().as_str() {
        "json" => Ok(OutputFormat::Json),
        "yaml" => Ok(OutputFormat::Yaml),
        _ => Err(anyhow::format_err!("Invalid output format: {}", format)),
    }
}
