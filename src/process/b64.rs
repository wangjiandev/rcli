use std::{
    fs::File,
    io::{stdin, Read},
};

use crate::cli::base64_options::Base64Format;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, prelude::BASE64_STANDARD, Engine as _};

pub fn process_encode(input: &str, format: &Base64Format) -> Result<(), anyhow::Error> {
    let mut reader: Box<dyn Read> = get_reader(input)?;

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let encoded = match format {
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buffer),
        Base64Format::Standard => BASE64_STANDARD.encode(buffer),
    };

    println!("{}", encoded);

    Ok(())
}

pub fn process_decode(input: &str, format: &Base64Format) -> Result<(), anyhow::Error> {
    let mut reader: Box<dyn Read> = get_reader(input)?;

    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    buffer = buffer.trim().to_owned();

    let decoded = match format {
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buffer)?,
        Base64Format::Standard => BASE64_STANDARD.decode(buffer)?,
    };

    let decoded = String::from_utf8(decoded)?;
    println!("{}", decoded.trim());
    Ok(())
}

fn get_reader(input: &str) -> Result<Box<dyn Read>, anyhow::Error> {
    if input == "-" {
        Ok(Box::new(stdin()))
    } else {
        Ok(Box::new(File::open(input)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "assets/tencode.txt";
        assert!(process_encode(input, &Base64Format::Standard).is_ok());
    }

    #[test]
    fn test_process_decode() {
        let input = "assets/tdecode.txt";
        assert!(process_decode(input, &Base64Format::Standard).is_ok());
    }
}
