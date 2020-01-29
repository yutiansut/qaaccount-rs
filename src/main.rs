

pub mod qaaccount;
pub mod qadata;
pub mod qafetch;
pub mod qaorder;
pub mod transaction;
use serde_json;
use std::error::Error;
use std::io;
use std::process;
extern crate stopwatch;
use stopwatch::{Stopwatch};

pub fn backtest(){




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

        qaaccount::QA_Account::send_order(&mut acc,bar.code.as_ref(), 10.0, bar.datetime.as_ref(), 2, bar.close, "order");

        //println!("{:?}", bar);
    }
    println!("{:?}", acc.history.len())

    //qaaccount::QA_Account::history_table(&mut acc);
}


fn main(){
    let sw = Stopwatch::start_new();
    backtest();
    println!("It took {0:.8} ms",sw.elapsed_ms());
}

