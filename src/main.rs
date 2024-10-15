use clap::Parser;
use rcli::{
    cli::{base64_options::Base64Command, text_options::TextCommand, Cli, Commands},
    process,
};

/// rcli csv -i input.csv -o output.json --header -d ','
/// rcli gen-pass --length 16 --uppercase --lowercase --numbers --symbols
/// rcli base64 encode -i input.txt
/// rcli base64 decode -i input.txt
/// rcli text sign -k fixtures/blake3
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Csv(options) => {
            process::process_csv(&options.input, &options.output, &options.format)?
        }
        Commands::GenPass(options) => process::process_genpass(&options)?,
        Commands::Base64(command) => match command {
            Base64Command::Encode(options) => {
                process::process_encode(&options.input, &options.format)?
            }
            Base64Command::Decode(options) => {
                process::process_decode(&options.input, &options.format)?
            }
        },
        Commands::Text(text_command) => match text_command {
            TextCommand::Sign(options) => {
                println!("TextCommand::Sign: {:?}", options);
            }
            TextCommand::Verify(options) => {
                println!("TextCommand::Verify: {:?}", options);
            }
            TextCommand::GenKey(options) => {
                println!("TextCommand::GenKey: {:?}", options);
            }
        },
    }
    Ok(())
}
