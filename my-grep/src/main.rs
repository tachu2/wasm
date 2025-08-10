use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    pattern: String,
    file_name: String,
}

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);
}
