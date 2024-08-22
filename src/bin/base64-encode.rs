use base64::prelude::*;
use clap::Parser;

/// base64-encodes the given input.
#[derive(Parser, Debug)]
#[command(about)]
struct Args {
    #[arg()]
    input: String,
}

fn main() {
    let args = Args::parse();
    let encoded_input = BASE64_STANDARD.encode(args.input);
    println!("{encoded_input}");
}
