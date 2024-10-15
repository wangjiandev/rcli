use clap::Args;

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
