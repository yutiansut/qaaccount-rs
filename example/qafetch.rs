use std::error::Error;
use std::io;
use std::process;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct BAR {
    code: String,
    datetime: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume:f64
}

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let bar: BAR = result?;
        println!("{:?}", bar);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}