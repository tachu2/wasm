use clap::Parser;
use wasmtime::{Engine, Linker, Module, Store};

#[derive(Parser, Debug)]
struct Args {
    wasm_file: String,
}

fn start(args: Args) -> anyhow::Result<()> {
    let engine = Engine::default();
    let component = Module::from_file(&engine, args.wasm_file)?;
    let linker = Linker::new(&engine);
    let mut store = Store::new(&engine, ());
    let instance = linker.instantiate(&mut store, &component)?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    if let Err(e) = start(args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
