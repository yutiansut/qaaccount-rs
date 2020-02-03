extern crate csv;
extern crate ndarray;
extern crate ndarray_csv;
extern crate num_traits;
extern crate serde;
extern crate stopwatch;

use std::borrow::BorrowMut;
use std::io;

use ndarray::{array, stack};
use stopwatch::Stopwatch;

use quantaxis_rs::{indicators, Next, qaaccount, qadata, qafetch, qaindicator, qaorder, transaction};
use quantaxis_rs::indicators::{
    BollingerBands, EfficiencyRatio, ExponentialMovingAverage, FastStochastic, HHV, LLV,
    Maximum, Minimum, MoneyFlowIndex, MovingAverage,
    MovingAverageConvergenceDivergence, OnBalanceVolume, RateOfChange, RelativeStrengthIndex, SimpleMovingAverage,
    SlowStochastic, StandardDeviation, TrueRange,
};
use quantaxis_rs::qaaccount::QA_Account;
use quantaxis_rs::qaorder::QA_Postions;
use std::cmp::{max, min};


use std::f64;

trait FloatIterExt {
    fn float_min(&mut self) -> f64;
    fn float_max(&mut self) -> f64;
}

impl<T> FloatIterExt for T where T: Iterator<Item=f64> {
    fn float_max(&mut self) -> f64 {
        self.fold(f64::NAN, f64::max)
    }

    fn float_min(&mut self) -> f64 {
        self.fold(f64::NAN, f64::min)
    }
}


pub fn backtest() -> QA_Account {
    let priceoffset = 1;
    let lossP = 1.3;
    let K1: usize = 20;
    let K2: usize = 20;
    let n1: usize = 30;

    let count1 = 0;
    let mut HAE: f64 = 0 as f64;
    let mut LAE: f64 = 0 as f64;
    let TrailingStart1 = 90;
    let TrailingStop1 = 10;
    let mut acc = qaaccount::QA_Account::new("BacktestAccount");
    acc.init_h("RBL8");
    let mut llv_i = LLV::new(K1 as u32).unwrap();
    let mut hhv_i = HHV::new(K2 as u32).unwrap();
    let mut ma = MovingAverage::new(n1 as u32).unwrap();
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut lastbar = qafetch::BAR {
        code: "".to_string(),
        datetime: "".to_string(),
        open: 0.0,
        high: 0.0,
        low: 0.0,
        close: 0.0,
        volume: 0.0
    };
    for result in rdr.deserialize() {
        let bar: qafetch::BAR = result.unwrap() ;
        let ind_llv = llv_i.next(bar.low);
        let ind_hhv = hhv_i.next(bar.high);
        let ind_ma = ma.next(bar.close);
        let crossOver = bar.high > hhv_i.cached[K1 - 2] && lastbar.high < hhv_i.cached[K1 - 2];

        let crossUnder = bar.low < llv_i.cached[K2 - 2] && lastbar.low > llv_i.cached[K2 - 2];

        let cond1 = ma.cached[n1 -1]> ma.cached[n1 -2] &&
                        ma.cached[n1 -2]> ma.cached[n1 -3] &&
            ma.cached[n1 - 3] > ma.cached[n1 - 4] &&
            ma.cached[n1 - 4] > ma.cached[n1 - 5];


        let cond2 = ma.cached[n1 - 1] < ma.cached[n1 - 2] &&
            ma.cached[n1 - 2] < ma.cached[n1 - 3] &&
            ma.cached[n1 - 3] < ma.cached[n1 - 4] &&
            ma.cached[n1 - 4] < ma.cached[n1 - 5];

        let code = bar.code.as_ref();

        let long_pos = acc.get_position_long(code);
        let short_pos = acc.get_position_short(code);
        if (long_pos > 0.0 || short_pos > 0.0) {

            if (HAE == 0.0){
                HAE = lastbar.high;
                LAE = lastbar.low;
            }else{
                if (HAE>= lastbar.high){
                    HAE = HAE
                }else{
                    HAE = lastbar.high
                }

                if (LAE <= lastbar.low){
                    LAE = LAE
                } else{
                    LAE = lastbar.low
                }
            }
        }

        if (long_pos == 0.0 && short_pos == 0.0) {
            if crossOver && cond1 {
                acc.buy_open(bar.code.as_ref(), 10.0, bar.datetime.as_ref(), bar.close);
            }
            if crossUnder && cond2 {
                acc.sell_open(bar.code.as_ref(), 10.0, bar.datetime.as_ref(), bar.close);
            }
        }
        if (long_pos > 0.0 && short_pos == 0.0) {
            //println!("当前多单持仓");

            let mut stopLine: f64 = acc.get_open_price_long(code) * (100.0 - lossP) / 100 as f64;
            if (HAE >= (acc.get_open_price_long(code) * (1 + TrailingStart1 / 1000) as f64)) {
                stopLine = (HAE * (1 - TrailingStop1 / 1000) as f64) as f64;
            }

            if (crossUnder && cond2) {
                acc.sell_close(code, 10.0, bar.datetime.as_ref(), bar.open);
            }
            if (bar.low < stopLine) {
                acc.sell_close(code, 10.0, bar.datetime.as_ref(), bar.open);
            }
        }
        if (short_pos > 0.0 && long_pos == 0.0) {
            //println!("当前空单持仓 {:#?}", acc.get_position_short(code));
            let mut stopLine: f64 = acc.get_open_price_short(code) * (100.0 + lossP) / 100 as f64;

            if (LAE >= (acc.get_open_price_short(code) * (1 - TrailingStart1 / 1000) as f64)) {
                stopLine = (LAE * (1 + TrailingStop1 / 1000) as f64) as f64;
            }
            if (crossOver && cond1) {
                acc.buy_close(code, 10.0, bar.datetime.as_ref(), bar.open);
            }
            if (bar.high < stopLine) {
                acc.buy_close(code, 10.0, bar.datetime.as_ref(), bar.open);
            }
        }


        lastbar = bar;
    }
    //println!("{:?}", acc.history_table());

    //qaaccount::QA_Account::history_table(&mut acc);

    acc
}


fn main(){
    let sw = Stopwatch::start_new();
    let acc = backtest();
    //let file = File::open("data15.csv").unwrap();
    
    println!("{:?}", acc.history_table());

    println!("It took {0:.8} ms", sw.elapsed_ms());
}
