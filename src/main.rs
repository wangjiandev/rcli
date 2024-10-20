use clap::Parser;
use rcli::{
    cli::{
        base64_options::Base64Command,
        text_options::{TextCommand, TextSignFormat},
        Cli, Commands,
    },
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
        Commands::GenPass(options) => {
            let password = process::process_genpass(&options)?;
            println!("{}", password);
        }
        Commands::Base64(command) => match command {
            Base64Command::Encode(options) => {
                let encoded = process::process_encode(&options.input, &options.format)?;
                println!("{}", encoded);
            }
            Base64Command::Decode(options) => {
                let decoded = process::process_decode(&options.input, &options.format)?;
                println!("{}", decoded);
            }
        },
        Commands::Text(text_command) => match text_command {
            TextCommand::Sign(options) => {
                let signature =
                    process::process_sign(&options.input, &options.key, options.format)?;
                println!("{}", signature);
            }
            TextCommand::Verify(options) => {
                let verified = process::process_verify(
                    &options.input,
                    &options.key,
                    options.format,
                    &options.signature,
                )?;
                println!("{}", verified);
            }
            TextCommand::GenKey(options) => {
                let keys = process::process_genkey(&options.format)?;
                match options.format {
                    TextSignFormat::Blake3 => {
                        let name = options.output.join("blake3.key");
                        std::fs::write(name, keys[0].clone())?;
                    }
                    TextSignFormat::Ed25519 => {
                        let sk_path = options.output.join("ed25519.key");
                        let pk_path = options.output.join("ed25519.pub");
                        std::fs::write(sk_path, keys[0].clone())?;
                        std::fs::write(pk_path, keys[1].clone())?;
                    }
                }
            }
        },
    }
    Ok(())
}
