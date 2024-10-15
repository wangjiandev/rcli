use clap::Parser;
use rcli::{
    cli::{base64_options::Base64Command, Cli, Commands},
    process,
};

/// rcli csv -i input.csv -o output.json --header -d ','
/// rcli gen-pass --length 16 --uppercase --lowercase --numbers --symbols
/// rcli base64 encode -i input.txt
/// rcli base64 decode -i input.txt
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
        Commands::Base64(command) => match command {
            Base64Command::Encode(options) => {
                process::process_encode(&options.input, &options.format)?
            }
            Base64Command::Decode(options) => {
                process::process_decode(&options.input, &options.format)?
            }
        },
    }
    Ok(())
}
