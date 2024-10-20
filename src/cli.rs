pub mod base64_options;
pub mod csv_options;
pub mod gen_pass_options;
pub mod text_options;

use anyhow::Result;
use base64_options::Base64Command;
use clap::{Parser, Subcommand};
use csv_options::CsvOptions;
use gen_pass_options::GenPassOptions;
use std::path::{Path, PathBuf};
use text_options::TextCommand;

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
    /// Encode or Decode Base64
    #[command(subcommand)]
    Base64(Base64Command),
    /// Text Sign or Verify
    #[command(subcommand)]
    Text(TextCommand),
}

fn verify_file(path: &str) -> Result<String> {
    if path == "-" {
        return Ok(path.into());
    }
    if !Path::new(path).exists() {
        return Err(anyhow::format_err!("Input file {} not found", path));
    }
    Ok(path.into())
}

fn verify_path(path: &str) -> Result<PathBuf> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err(anyhow::format_err!("Output file {} dose not exist", path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_file() {
        assert!(matches!(verify_file("-"), Ok(s) if s == "-"));
        assert!(matches!(verify_file("Cargo.toml"), Ok(s) if s == "Cargo.toml"));
        assert!(verify_file("*").is_err());
        assert!(verify_file("Cargo.toml").is_ok());
        assert!(verify_file("not_exist.txt").is_err());
    }
}
