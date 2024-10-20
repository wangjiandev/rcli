use super::verify_path;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub enum HttpCommand {
    /// Serve a directory over HTTP
    #[command(name = "serve")]
    Serve(HttpServerOptions),
}

#[derive(Parser, Debug)]
pub struct HttpServerOptions {
    /// Directory to serve
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub directory: PathBuf,

    /// Port to serve on
    #[arg(short, long, default_value = "8080")]
    pub port: u16,
}
