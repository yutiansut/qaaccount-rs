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


pub struct QABacktest {}

impl QABacktest {
    fn create() -> Self {
        let backtest = QABacktest {};
        backtest
    }

    fn init(&mut self) {
        
    }

    fn on_bar(&mut self, bar: qafetch::BAR) {}

    fn run(&mut self) {}

    fn day_open(&mut self) {}

    fn day_close(&mut self) {}

    fn on_backtest_close(&mut self) {}
}
pub fn main() {

}