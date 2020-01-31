//count1=0
//HAE=0
//LAE=0
//
//def calc_HHV(self,market_data, n):
//try:
//ind = QA.HHV(market_data.iloc[-n-1:]['high'],n)
//return ind.iloc[-2]
//except Exception as e:
//print(e)
//return np.nan
//
//def calc_LLV(self,market_data, n):
//try:
//ind = QA.LLV(market_data.iloc[-n-1:]['low'],n)
//return ind.iloc[-2]
//except Exception as e:
//print(e)
//return np.nan
//

use quantaxis_rs::{qaaccount, qafetch, qaindicator, qadata, qaorder, transaction};

extern crate serde;
extern crate num_traits;


extern crate csv;
extern crate ndarray;
use ndarray::{array, stack};

use ndarray::prelude::*;
extern crate ndarray_csv;

use csv::{ReaderBuilder, WriterBuilder};
use ndarray::{Array, Array2};
use ndarray_csv::{Array2Reader, Array2Writer};
use std::fs::File;

use serde_json;
use std::error::Error;
use std::io;
use std::process;
extern crate stopwatch;
use stopwatch::{Stopwatch};


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
    let dh =  array!(&init_data);


    let mut acc  = qaaccount::QA_Account {
        cash: vec![],
        hold: vec![],
        history: vec![],
        account_cookie: "x".to_string(),
        portfolio_cookie: "x".to_string(),
        user_cookie: "x".to_string()
    };

    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let bar: qafetch::BAR = result.unwrap() ;

//        let bx =  Array::from(bar);
//        let dh = stack(Axis(1), &[dh.view(), bx.view()]);
//        println!("{:#?}", dh);


        qaaccount::QA_Account::send_order(&mut acc,bar.code.as_ref(), 10.0, bar.datetime.as_ref(), 2, bar.close, "order");

        //println!("{:?}", bar);
    }
    println!("{:?}", acc.history.len())

    //qaaccount::QA_Account::history_table(&mut acc);
}


fn main(){
    let sw = Stopwatch::start_new();
    backtest();

    //let file = File::open("data15.csv").unwrap();


    println!("It took {0:.8} ms",sw.elapsed_ms());
}
