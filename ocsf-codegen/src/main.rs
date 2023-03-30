//! Runs the codegen process
//!

#[macro_use]
extern crate log;

use env_logger::{Builder, Target};
use ocsf_codegen::*;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    dirpath: Option<String>,
}

pub fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "DEBUG");
    }

    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout).format_timestamp(None).init();

    let cli = Cli::parse();

    let base_path = match cli.dirpath {
        Some(val) => val,
        None => String::from("./"),
    };

    if let Err(err) = generate_source_code(&base_path) {
        error!("Failed to do the thing! {:?}", err);
        std::process::exit(1);
    }
}
