extern crate ndarray;
extern crate ndarray_csv;
extern crate num_traits;
extern crate serde;
extern crate stopwatch;

use std::error::Error;
use std::io;
use std::process;

use csv::{ReaderBuilder, WriterBuilder};
use ndarray::{array, stack};
use ndarray::prelude::*;
use serde_json;
use stopwatch::Stopwatch;

pub mod qaaccount;
pub mod qadata;
pub mod qafetch;
pub mod qaorder;
pub mod qaindicator;
pub mod transaction;


//use ndarray::{Array, Array2};
//use ndarray_csv::{Array2Reader, Array2Writer};
//use std::fs::File;

pub fn backtest(){
    let init_data = qafetch::BAR{
        code: "".to_string(),
        datetime: "".to_string(),
        open: 0.0,
        high: 0.0,
        low: 0.0,
        close: 0.0,
        volume: 0.0
    };
    let dh = array!(&init_data);
    let mut acc = qaaccount::QA_Account::new("mainAcc");
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        let bar: qafetch::BAR = result.unwrap() ;
        qaaccount::QA_Account::send_order(&mut acc,bar.code.as_ref(), 10.0, bar.datetime.as_ref(), 2, bar.close, "order");
    }
    println!("{:?}", acc.history.len());
}


fn main(){
    let sw = Stopwatch::start_new();

    println!("It took {0:.8} ms",sw.elapsed_ms());
}

