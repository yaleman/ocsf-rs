use ocsf_codegen::generate_scope;

use clap::Parser;


#[derive(Parser)]
struct Cli {
    #[arg(short,long)]
    dirpath: Option<String>,
}


pub fn main() {
    let cli = Cli::parse();

    let base_path = match cli.dirpath {
        Some(val) => val,
        None => String::from("../"),
    };

    generate_scope(&base_path).map_err(|err| {
        eprintln!("Failed to do the thing! {err:?}");
    }).unwrap();
}
