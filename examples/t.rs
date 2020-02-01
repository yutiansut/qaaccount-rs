
use quantaxis_rs::{qaaccount, qafetch, qaindicator, qadata, qaorder, indicators, transaction, Next};
use quantaxis_rs::indicators::{
    BollingerBands, EfficiencyRatio, ExponentialMovingAverage, FastStochastic, Maximum, Minimum,
    MoneyFlowIndex, MovingAverageConvergenceDivergence, OnBalanceVolume, RateOfChange,
    RelativeStrengthIndex, SimpleMovingAverage, SlowStochastic, StandardDeviation, TrueRange,
    LLV, HHV, MovingAverage
};
extern crate serde;
extern crate num_traits;
extern crate csv;
extern crate ndarray;
use ndarray::{array, stack};
extern crate ndarray_csv;
use std::io;
extern crate stopwatch;
use stopwatch::{Stopwatch};
use quantaxis_rs::qaaccount::QA_Account;
use std::borrow::BorrowMut;
use quantaxis_rs::qaorder::QA_Postions;


pub fn backtest(){
    let priceoffset = 1;
    let lossP = 1.3;
    let K1 = 20;
    let K2 = 20;
    let n1:usize = 30;

    let count1=0;
    let HAE=0;
    let LAE=0;
    let TrailingStart1 = 90;
    let TrailingStop1 = 10;
    let init_data = qafetch::BAR{
        code: "".to_string(),
        datetime: "".to_string(),
        open: 0.0,
        high: 0.0,
        low: 0.0,
        close: 0.0,
        volume: 0.0
    };
    let mut acc  = qaaccount::QA_Account::new();
    acc.init_h("RB2005");
    let mut llv_i = LLV::new(3).unwrap();
    let mut hhv_i = HHV::new(3).unwrap();
    let mut ma = MovingAverage::new(n1 as u32).unwrap();
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut lastbar = qafetch::BAR{
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
        let crossOver = bar.high > hhv_i.cached[1] && lastbar.high < hhv_i.cached[1];

        let crossUnder = bar.low < llv_i.cached[1] && lastbar.low > llv_i.cached[1];

        let cond1 = ma.cached[n1 -1]> ma.cached[n1 -2] &&
                        ma.cached[n1 -2]> ma.cached[n1 -3] &&
                        ma.cached[n1 -3]> ma.cached[n1 - 4] &&
                        ma.cached[n1-4]> ma.cached[n1-5];


        let cond2 = ma.cached[n1 -1]< ma.cached[n1 -2] &&
            ma.cached[n1 -2]< ma.cached[n1 -3] &&
            ma.cached[n1 -3]< ma.cached[n1 - 4] &&
            ma.cached[n1-4]< ma.cached[n1-5];

        let code = bar.code.as_ref();


        if acc.get_position_long(code) == 0 as f64 && acc.get_position_short(code) == 0 as f64 {
            if crossOver && cond1{
                acc.buy_open(bar.code.as_ref(), 10.0, bar.datetime.as_ref(), bar.close);
            }
            if crossUnder && cond2{
               acc.sell_open(bar.code.as_ref(), 10.0, bar.datetime.as_ref(), bar.close);
            }
        }
        if acc.get_position_long(code) >=0 as f64 && acc.get_position_short(code) == 0 as f64{
            println!("当前多单持仓");



//                Stopline = round(self.positions.open_price_long*(100-lossP)/100,0)
//                if (self.HAE >= self.positions.open_price_long*(1+TrailingStart1/1000) and bar_id-self.count1 >= 1):
//                    Stopline = self.HAE*(1-TrailingStop1/1000)
//
//                if CrossUnder and cond2:
//                    self.send_order('SELL', 'CLOSE', price=min(bar['open'], LLV) - priceoffset, volume= self.positions.volume_long)
//                    print('SELL_CLOSE')
//
//                elif bar['low'] <= Stopline:
//                    self.send_order('SELL', 'CLOSE', price=min(bar['open'], Stopline) - priceoffset, volume= self.positions.volume_long)
//                    print('SELL_CLOSE_STOPLOSS')

        }
        if acc.get_position_short(code) >=0 as f64 && acc.get_position_long(code) == 0 as f64{
            println!("当前空单持仓");

        }



        lastbar = bar;


    }
    println!("{:?}", acc.history_table());

    //qaaccount::QA_Account::history_table(&mut acc);
}


fn main(){
    let sw = Stopwatch::start_new();
    backtest();
    //let file = File::open("data15.csv").unwrap();
    println!("It took {0:.8} ms",sw.elapsed_ms());
}
