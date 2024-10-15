use std::{fmt::Display, str::FromStr};

use clap::{Args, Parser};

use super::verify_input_file;

#[derive(Parser, Debug)]
pub enum Base64Command {
    /// Encode
    #[command(name = "encode")]
    Encode(Base64EncodeOptions),

    /// Decode
    #[command(name = "decode")]
    Decode(Base64DecodeOptions),
}

#[derive(Args, Debug)]
pub struct Base64EncodeOptions {
    /// Input file path
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,

    /// Base64 format
    #[arg(short, long, value_parser = parser_base64_format, default_value = "url_safe")]
    pub format: Base64Format,
}

#[derive(Args, Debug)]
pub struct Base64DecodeOptions {
    /// Input file path
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,

    /// Base64 format
    #[arg(short, long, value_parser = parser_base64_format, default_value = "url_safe")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    /// URL Safe
    UrlSafe,
    /// Standard
    Standard,
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "url_safe" => Ok(Base64Format::UrlSafe),
            "standard" => Ok(Base64Format::Standard),
            _ => Err(anyhow::anyhow!("Invalid base64 format: {}", s)),
        }
    }
}

impl From<Base64Format> for String {
    fn from(value: Base64Format) -> Self {
        match value {
            Base64Format::UrlSafe => "url_safe".to_string(),
            Base64Format::Standard => "standard".to_string(),
        }
    }
}

impl Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(*self))
    }
}

fn parser_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}
