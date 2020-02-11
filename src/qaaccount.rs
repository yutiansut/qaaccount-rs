use std::borrow::Borrow;
use std::collections::HashMap;
use std::error::Error;
use std::io;

use csv;
use uuid::Uuid;

use crate::market_preset::{CodePreset, MarketPreset};
use crate::qaorder::QAOrder;
use crate::qaposition;
use crate::qaposition::{QA_Frozen, QA_Postions};
use crate::transaction;
use crate::transaction::QATransaction;

#[warn(non_camel_case_types)]
pub struct QA_Account {
    init_cash: f64,
    init_hold: HashMap<String, QA_Postions>,

    allow_t0: bool,
    allow_sellopen: bool,
    allow_margin: bool,

    auto_reload: bool,
    market_preset: MarketPreset,

    pub cash: Vec<f64>,
    pub money: f64,
    pub hold: HashMap<String, QA_Postions>,
    pub frozen: HashMap<String, QA_Frozen>,
    pub history: Vec<transaction::QATransaction>,
    pub account_cookie: String,
    pub portfolio_cookie: String,
    pub user_cookie: String,
    environment: String,
    close_profit: f64,
}

impl QA_Account {
    pub fn new(account_cookie: &str,
               portfolio_cookie: &str,
               user_cookie: &str,
               init_cash: f64,
               auto_reload: bool,
               environment: &str) -> Self {
        let mut acc = Self {
            init_cash,
            init_hold: HashMap::new(),
            allow_t0: false,
            allow_sellopen: false,
            allow_margin: false,
            market_preset: MarketPreset::new(),
            auto_reload,
            cash: vec![init_cash],
            money: init_cash,
            hold: HashMap::new(),
            frozen: HashMap::new(),
            history: vec![],
            account_cookie: account_cookie.parse().unwrap(),
            portfolio_cookie: portfolio_cookie.parse().unwrap(),
            user_cookie: user_cookie.parse().unwrap(),
            environment: environment.to_string(),
            close_profit: 0.0,
        };

        if auto_reload {
            acc.reload()
        }
        acc
    }
    pub fn init_h(&mut self, code: &str) {
        let code: String = code.parse().unwrap();
        self.hold.insert(code.clone(), QA_Postions::new(code.clone(), self.account_cookie.clone(),
                                                        self.account_cookie.clone(), self.account_cookie.clone(),
                                                        self.portfolio_cookie.clone()));
    }

    pub fn reload(&mut self) {}

    /// positions about
    ///
    /// a fast way to get the realtime price/cost/volume/history
    pub fn get_position(&mut self, code: &str) -> Option<&mut QA_Postions> {
        let pos = self.hold.get_mut(code);
        pos
    }
    pub fn get_volume_long(&mut self, code: &str) -> f64 {
        let pos = self.get_position(code).unwrap();
        pos.volume_long()
    }
    pub fn get_volume_short(&mut self, code: &str) -> f64 {
        let pos = self.get_position(code).unwrap();
        pos.volume_short()
    }
    pub fn get_open_price_long(&mut self, code: &str) -> f64 {
        self.get_position(code).unwrap().open_price_long
    }
    pub fn get_open_price_short(&mut self, code: &str) -> f64 {
        self.get_position(code).unwrap().open_price_short
    }


    pub fn get_frozen(&mut self, code: &str) -> f64 {
        self.get_position(code).unwrap().frozen
    }

    pub fn settle(&mut self) {}
    /// history about

    pub fn history_table(&self) {
        for item in self.history.iter() {
            println!("{:?}", transaction::QATransaction::to_json(item));
        }
    }
    pub fn to_csv(&self) -> Result<(), Box<dyn Error>> {
        let mut wtr = csv::Writer::from_writer(io::stdout());
        for item in self.history.iter() {
            wtr.serialize(item)?;
            wtr.flush()?;
        }
        Ok(())
    }

    /// order about
    /// buy| sell| buy_open| sell_open| buy_close| sell_close|
    /// send_order
    pub fn buy(
        &mut self,
        code: &str,
        amount: f64,
        time: &str,
        price: f64,
    ) {
        self.send_order(code, amount, time, 1, price, "BUY");
    }
    pub fn sell(
        &mut self,
        code: &str,
        amount: f64,
        time: &str,
        price:f64
    ) {
        self.send_order(code, amount, time, -1, price, "SELL");
    }
    pub fn buy_open(
        &mut self,
        code: &str,
        amount: f64,
        time: &str,
        price:f64
    ) {
        self.send_order(code, amount, time, 2, price, "BUY_OPEN");
    }
    pub fn sell_open(
        &mut self,
        code: &str,
        amount: f64,
        time: &str,
        price:f64
    ) {
        self.send_order(code, amount, time, -2, price, "SELL_OPEN");
    }
    pub fn buy_close(
        &mut self,
        code: &str,
        amount: f64,
        time: &str,
        price:f64
    ) {
        self.send_order(code, amount, time, 3, price, "BUY_CLOSE");
    }
    pub fn sell_close(
        &mut self,
        code: &str,
        amount: f64,
        time: &str,
        price: f64,
    ) {
        self.send_order(code, amount, time, -3, price, "SELL_CLOSE");
    }


    fn order_check(&mut self, code: &str, amount: f64, price: f64, towards: i32, order_id: String) -> bool {
        let mut res = false;

        let qapos = self.get_position(code).unwrap();

        match towards {
            3 => {
                if (qapos.volume_short() - qapos.volume_short_frozen()) >= amount {
                    qapos.volume_short_frozen_today += amount;
                    qapos.volume_short_today -= amount;
                    res = true;
                } else {
                    println!("仓位不足");
                }
            }
            4 => {
                if (qapos.volume_short_today - qapos.volume_short_frozen_today) >= amount {
                    qapos.volume_short_frozen_today += amount;
                    qapos.volume_short_today -= amount;
                    res = true;
                } else {
                    println!("今日仓位不足");
                }
            }

            -3 => {
                if (qapos.volume_long() - qapos.volume_long_frozen()) >= amount {
                    qapos.volume_long_frozen_today += amount;
                    qapos.volume_long_today -= amount;
                    res = true;
                } else {
                    println!("SELL CLOSE 仓位不足");
                }
            }

            -4 => {
                if (qapos.volume_long_today - qapos.volume_short_frozen_today) >= amount {
                    qapos.volume_long_frozen_today += amount;
                    qapos.volume_long_today -= amount;
                    res = true;
                } else {
                    println!("SELL CLOSETODAY 仓位不足");
                }
            }

            1 | 2 | -2 => {
                let coeff = qapos.preset.calc_coeff() * price;
                println!("{:#?}", coeff);
                let frozen = coeff * amount;
                if self.money > frozen {
                    self.money -= frozen;
                    self.frozen.insert(order_id, QA_Frozen {
                        amount,
                        coeff,
                        money: frozen,
                    });
                    res = true
                } else {
                    println!("余额不足");
                }
            }
            _ => {}
        }
        res
    }

    pub fn send_order(
        &mut self,
        code: &str,
        amount: f64,
        time: &str,
        towards: i32,
        price: f64,
        order_id: &str,
    ) -> Result<QAOrder, ()> {
        let order_id: String = Uuid::new_v4().to_string();
        if self.order_check(code, amount, price, towards, order_id.clone()) {
            let order = QAOrder::new(self.account_cookie.clone(), code.to_string(),
                                     towards, "".to_string(), "".to_string(),
                                     amount, price, order_id.clone());

            if self.environment == "backtest" {
                self.receive_deal(code.parse().unwrap(), amount, price, time.parse().unwrap(),
                                  order_id.clone(), order_id.clone(), order_id.clone(),
                                  towards)
            }
            Ok(order.clone())
        } else {
            Err(())
        }
    }


    fn receive_deal(&mut self, code: String, amount: f64, price: f64, datetime: String,
                    order_id: String, trade_id: String, realorder_id: String, towards: i32,
    ) {

        //            self.trades[trade_id] = {
        //                "seqno": self.event_id,
        //                "user_id":  self.user_id,
        //                "trade_id": trade_id,
        //                "exchange_id": od['exchange_id'],
        //                "instrument_id": od['instrument_id'],
        //                "order_id": order_id,
        //                "exchange_trade_id": trade_id,
        //                "direction": od['direction'],
        //                "offset": od['offset'],
        //                "volume": trade_amount,
        //                "price": trade_price,
        //                "trade_time": trade_time,
        //                "trade_date_time": self.transform_dt(trade_time)}


        if self.frozen.contains_key(&order_id) {
            let frozen = self.frozen.get_mut(&order_id).unwrap();
            self.money += frozen.money;
            self.frozen.insert(order_id.clone(), QA_Frozen {
                amount: 0.0,
                coeff: 0.0,
                money: 0.0,
            });
        }


        let qapos = self.get_position(code.as_ref()).unwrap();
        let (margin, close_profit) = qapos.update_pos(price, amount, towards);
        self.money -= (margin - close_profit);
        self.close_profit += close_profit;
        self.cash.push(self.money);
        self.history.push(transaction::QATransaction {
            code,
            amount,
            price,
            datetime,
            order_id,
            trade_id,
            realorder_id,
            account_cookie: self.account_cookie.clone(),
            commission: 0.0,
            tax: 0.0,
            message: "".to_string(),
            frozen: 0.0,
            direction: towards,
        });
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mut acc = QA_Account::new("RustT01B2_RBL8", "test", "admin",
                                      100000.0, false, "backtest");
        acc.history_table();
    }
    #[test]
    fn test_pos() {
        let mut acc = QA_Account::new("RustT01B2_RBL8", "test", "admin",
                                      100000.0, false, "backtest");
        acc.init_h("RB2005");
        acc.get_position("RB2005");
    }

    #[test]
    fn test_init_h() {
        let mut acc = QA_Account::new("RustT01B2_RBL8", "test", "admin",
                                      100000.0, false, "backtest");
        acc.init_h("RB2005");
        println!("{:#?}", acc.get_position("RB2005").unwrap().message());
    }

    #[test]
    fn test_buy_open() {
        println!("test buy open");
        let code = "RB2005";

        let mut acc = QA_Account::new("RustT01B2_RBL8", "test", "admin",
                                      1000000.0, false, "backtest");
        acc.init_h(code);
        acc.buy_open(code, 10.0, "2020-01-20", 3500.0);
        println!("MONEY LEFT{:#?}", acc.money);
        assert_eq!(acc.get_volume_long(code), 10.0);
        //println!("{:#?}", )
        acc.history_table();
    }

    #[test]
    fn test_sell_open() {
        println!("test sell open");
        let code = "RB2005";

        let mut acc = QA_Account::new("RustT01B2_RBL8", "test", "admin",
                                      100000.0, false, "backtest");
        acc.init_h(code);
        acc.sell_open(code, 10.0, "2020-01-20", 3500.0);

        assert_eq!(acc.get_volume_short(code), 10.0);

        //assert_eq!(acc.)
        //println!("{:#?}", )
        acc.history_table();
    }


    #[test]
    fn test_buy_close() {
        println!("test buy close");
        let code = "RB2005";

        let mut acc = QA_Account::new("RustT01B2_RBL8", "test", "admin",
                                      100000.0, false, "backtest");
        acc.init_h(code);
        acc.sell_open(code, 10.0, "2020-01-20", 3500.0);

        assert_eq!(acc.get_volume_short(code), 10.0);
        acc.buy_close(code, 10.0, "2020-01-20", 3600.0);
        assert_eq!(acc.get_volume_short(code), 0.0);

        println!("after all {:#?}", acc.money);
        acc.history_table();
    }

    #[test]
    fn test_sell_close() {
        println!("test sell close");
        let code = "RB2005";

        let mut acc = QA_Account::new("RustT01B2_RBL8", "test", "admin",
                                      100000.0, false, "backtest");
        acc.init_h(code);
        acc.buy_open(code, 10.0, "2020-01-20", 3500.0);
        assert_eq!(acc.get_volume_long(code), 10.0);
        acc.sell_close(code, 10.0, "2020-01-20", 3600.0);

        assert_eq!(acc.get_volume_long(code), 0.0);
        //println!("{:#?}", )
        println!("{:#?}", acc.money);
        acc.history_table();
    }
}