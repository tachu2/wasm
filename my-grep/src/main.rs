use std::{fs::File, io::{BufRead, BufReader}};

use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    pattern: String,
    file_name: String,
}

fn start(cli: Cli) -> anyhow::Result<()> {
    let file = File::open(cli.file_name)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if line.contains(&cli.pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}

fn main() {
    let args = Cli::parse();
    start(args).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });
}
