use clap::Parser;
use rcli::{
    options::{Cli, Commands},
    process,
};

/// rcli csv -i input.csv -o output.json --header -d ','
/// rcli gen-pass --length 16 --uppercase --lowercase --numbers --symbols
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Csv(options) => {
            println!("CsvOptions: {:?}", options);
            process::process_csv(&options.input, &options.output, &options.format)?
        }
        Commands::GenPass(options) => {
            println!("GenPassOptions: {:?}", options);
            process::process_genpass(&options)?
        }
    }
    Ok(())
}
