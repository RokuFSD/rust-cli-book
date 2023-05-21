use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    #[arg(short, long)]
    pattern: String,
    /// The path to the file to read
    path: PathBuf,
}

fn main() -> Result<(), ()> {
    let mut handle = io::stdout().lock();
    let mut handle_err = io::stderr().lock();

    let args = Cli::parse();
    let f = File::open(&args.path).map_err(|err| {
        writeln!(handle_err, "Error opening file: {}", err).unwrap();
    })?;

    let reader = BufReader::new(f);
    let lines = reader.lines();

    for line in lines {
        let str_line = line.map_err(|err| {
            writeln!(handle_err, "Error reading line: {}", err).unwrap();
        })?;
        rust_cli_book::find_match(&str_line, &args.pattern, &mut handle);
    }

    Ok(())
}
