use rand::Rng;

use crate::options::GenPassOptions;

pub fn process_genpass(options: &GenPassOptions) -> anyhow::Result<()> {
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

    println!("{}", password);

    Ok(())
}
