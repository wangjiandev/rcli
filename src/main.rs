use clap::Parser;
use rcli::{
    options::{Cli, Commands},
    process,
};

/// rcli csv -i input.csv -o output.json --header -d ','
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Csv(options) => process::process_csv(
            &options.input,
            &options.output,
            options.header,
            options.delimiter,
        )?,
    }
    Ok(())
}
