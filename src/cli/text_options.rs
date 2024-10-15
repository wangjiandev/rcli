use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use super::verify_file;
use clap::{Args, Parser};

#[derive(Parser, Debug)]
pub enum TextCommand {
    /// Sign a message with a private/shared key
    #[command(name = "sign")]
    Sign(TextSignOptions),

    /// Verify a message with a public key
    #[command(name = "verify")]
    Verify(TextVerifyOptions),

    /// GenKey generate a private/shared key
    #[command(name = "genkey")]
    GenKey(TextGenKeyOptions),
}

#[derive(Args, Debug)]
pub struct TextSignOptions {
    /// Input file path
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    /// Private key file path
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub key: String,

    /// Text sign format
    #[arg(short, long, value_parser = parser_text_sign_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Args, Debug)]
pub struct TextVerifyOptions {
    /// Input file path
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    /// Public key file path
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub key: String,

    /// Signature file path
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub signature: String,
}

#[derive(Args, Debug)]
pub struct TextGenKeyOptions {}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid text sign format")),
        }
    }
}

fn parser_text_sign_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl Display for TextSignFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
