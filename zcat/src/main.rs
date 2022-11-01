use std::error::Error;
use clap::Parser;

mod config;

type ZCatResult<T> = Result<T, Box<dyn Error>>;

fn main() -> ZCatResult<()> {
    let args = config::ZCatArguments::parse();
    println!("Hello, world! {}", args.number_lines);
    return Ok(());
}
