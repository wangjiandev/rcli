use crate::cli::gen_pass_options::GenPassOptions;
use anyhow::Result;
use rand::Rng;

pub fn process_genpass(options: &GenPassOptions) -> Result<String> {
    let mut rng = rand::thread_rng();
    let mut password = String::new();
    let mut chars = Vec::new();
    if !options.no_uppercase {
        chars.extend_from_slice(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if !options.no_lowercase {
        chars.extend_from_slice(b"abcdefghijklmnpqrstuvwxyz");
    }
    if !options.no_numbers {
        chars.extend_from_slice(b"0123456789");
    }
    if !options.no_symbols {
        chars.extend_from_slice(b"@#$%^&*?");
    }
    for _ in 0..options.length {
        let index = rng.gen_range(0..chars.len());
        password.push(chars[index] as char);
    }
    Ok(password)
}
