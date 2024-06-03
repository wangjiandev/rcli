use rand::prelude::SliceRandom;

const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBER: &[u8] = b"0123456789";
const SYMBOL: &[u8] = b"!@#$%?&*";

pub fn process_genpass(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if uppercase {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("UPPER is empty"));
    }
    if lowercase {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("UPPER is empty"));
    }
    if number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("UPPER is empty"));
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("UPPER is empty"));
    }

    for _ in 0..(length - password.len() as u8) {
        let c = *chars.choose(&mut rng).expect("chars is empty");
        password.push(c);
    }
    password.shuffle(&mut rng);

    let password = String::from_utf8(password)?;
    println!("Generated password: {}", password);

    let estimate = zxcvbn::zxcvbn(password.as_str(), &[]); // zxcvbn is a password strength estimator
    println!("Password Estimate: {}", estimate.score());

    Ok(())
}
