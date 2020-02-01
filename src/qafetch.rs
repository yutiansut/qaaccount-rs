use std::error::Error;
use std::io;
use std::process;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BAR {
    pub code: String,
    pub datetime: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume:f64
}

impl BAR{
    pub fn print(&self){
        println!("{:#?} -{:#?} ", self.datetime, self.open)
    }
}