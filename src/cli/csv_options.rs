use clap::Args;

use super::verify_file;

#[derive(Args, Debug)]
pub struct CsvOptions {
    /// Input CSV file path
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
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

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

fn parse_output_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    match format.to_lowercase().as_str() {
        "json" => Ok(OutputFormat::Json),
        "yaml" => Ok(OutputFormat::Yaml),
        _ => Err(anyhow::format_err!("Invalid output format: {}", format)),
    }
}
