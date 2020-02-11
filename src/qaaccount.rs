use std::collections::HashMap;
use std::error::Error;
use std::io;

use csv;
use uuid::Uuid;

use crate::market_preset::{CodePreset, MarketPreset};
use crate::qaorder::QAOrder;
use crate::qaposition;
use crate::qaposition::QA_Postions;
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
    pub hold: HashMap<String, QA_Postions>,

    pub history: Vec<transaction::QATransaction>,
    pub account_cookie: String,
    pub portfolio_cookie: String,
    pub user_cookie: String,
}

impl QA_Account {
    pub fn new(account_cookie: &str,
               portfolio_cookie: &str,
               user_cookie: &str,
               init_cash: f64,
               auto_reload: bool, ) -> Self {
        let mut acc = Self {
            init_cash,
            init_hold: HashMap::new(),
            allow_t0: false,
            allow_sellopen: false,
            allow_margin: false,
            market_preset: MarketPreset::new(),
            auto_reload,
            cash: vec![],
            hold: HashMap::new(),
            history: vec![],
            account_cookie: account_cookie.parse().unwrap(),
            portfolio_cookie: portfolio_cookie.parse().unwrap(),
            user_cookie: user_cookie.parse().unwrap(),
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
//
//    def order_check(self, code: str, amount: float, price: float, towards: int, order_id: str) -> bool:
//    res = False
//    qapos = self.get_position(code)
//
//    self.log(qapos.curpos)
//    self.log(qapos.close_available)
//    if towards == ORDER_DIRECTION.BUY_CLOSE:
//    # self.log("buyclose")
//    # self.log(self.volume_short - self.volume_short_frozen)
//    # self.log(amount)
//    if (qapos.volume_short - qapos.volume_short_frozen) >= amount:
//    # check
//    qapos.volume_short_frozen_today += amount
//    qapos.volume_short_today -= amount
//    res = True
//    else:
//    self.log("BUYCLOSE 仓位不足")
//
//    elif towards == ORDER_DIRECTION.BUY_CLOSETODAY:
//    if (qapos.volume_short_today - qapos.volume_short_frozen_today) >= amount:
//    qapos.volume_short_frozen_today += amount
//    qapos.volume_short_today -= amount
//    res = True
//    else:
//    self.log("BUYCLOSETODAY 今日仓位不足")
//    elif towards == ORDER_DIRECTION.SELL_CLOSE:
//    # self.log("sellclose")
//    # self.log(self.volume_long - self.volume_long_frozen)
//    # self.log(amount)
//    if (qapos.volume_long - qapos.volume_long_frozen) >= amount:
//    qapos.volume_long_frozen_today += amount
//    qapos.volume_long_today -= amount
//    res = True
//    else:
//    self.log("SELL CLOSE 仓位不足")
//
//    elif towards == ORDER_DIRECTION.SELL_CLOSETODAY:
//    if (qapos.volume_long_today - qapos.volume_short_frozen_today) >= amount:
//    # self.log("sellclosetoday")
//    # self.log(self.volume_long_today - self.volume_long_frozen)
//    # self.log(amount)
//    qapos.volume_long_frozen_today += amount
//    qapos.volume_long_today -= amount
//    return True
//    else:
//    self.log("SELLCLOSETODAY 今日仓位不足")
//    elif towards in [ORDER_DIRECTION.BUY_OPEN,
//    ORDER_DIRECTION.SELL_OPEN,
//    ORDER_DIRECTION.BUY]:
//    """
//            冻结的保证金
//            """
//    coeff = float(price) * float(
//    self.market_preset.get_code(code).get("unit_table",
//    1)
//    ) * float(self.market_preset.get_code(code).get("buy_frozen_coeff",
//    1))
//    moneyneed = coeff * amount
//    if self.available > moneyneed:
//    self.money -= moneyneed
//    self.frozen[order_id] = {
//    'amount': amount,
//    'coeff': coeff,
//    'money': moneyneed
//    }
//    res = True
//    else:
//    self.log("开仓保证金不足 TOWARDS{} Need{} HAVE{}".format(
//    towards, moneyneed, self.available))
//
//    return res


    fn order_check(&mut self, code: &str, amount: f64, price: f64, towards: i32) -> bool {
        let pos = self.get_position(code).unwrap();
        let (mv, pro) = pos.update_pos(price, amount, towards);
        println!("MARGIN VALUE {:#?} profit {:#?}", mv, pro);
        true
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
        if self.order_check(code, amount, price, towards) {
            let order = QAOrder::new(self.account_cookie.clone(), code.to_string(),
                                     towards, "".to_string(), "".to_string(),
                                     amount, price);
            Ok(order.clone())
        } else {
            Err(())
        }
    }

    fn receive_deal(&mut self, code: String, amount: f64, price: f64, datetime: String,
                    order_id: String, trade_id: String, realorder_id: String, towards: i32,
    ) {
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
                                      100000.0, false);
        acc.history_table();
    }
    #[test]
    fn test_pos() {
        let mut acc = QA_Account::new("RustT01B2_RBL8", "test", "admin",
                                      100000.0, false);
        acc.init_h("RB2005");
        acc.get_position("RB2005");
    }

    #[test]
    fn test_init_h() {
        let mut acc = QA_Account::new("RustT01B2_RBL8", "test", "admin",
                                      100000.0, false);
        acc.init_h("RB2005");
        println!("{:#?}", acc.get_position("RB2005").unwrap().message());
    }

    #[test]
    fn test_buy_open() {
        println!("test buy open");
        let code = "RB2005";

        let mut acc = QA_Account::new("RustT01B2_RBL8", "test", "admin",
                                      100000.0, false);
        acc.init_h(code);
        acc.buy_open(code, 10.0, "2020-01-20", 3500.0);

        assert_eq!(acc.get_volume_long(code), 10.0);
        //println!("{:#?}", )
        acc.history_table();
    }

    #[test]
    fn test_sell_open() {
        println!("test sell open");
        let code = "RB2005";

        let mut acc = QA_Account::new("RustT01B2_RBL8", "test", "admin",
                                      100000.0, false);
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
                                      100000.0, false);
        acc.init_h(code);
        acc.sell_open(code, 10.0, "2020-01-20", 3500.0);

        assert_eq!(acc.get_volume_short(code), 10.0);
        acc.buy_close(code, 10.0, "2020-01-20", 3600.0);
        assert_eq!(acc.get_volume_short(code), 0.0);

        //println!("{:#?}", )
        acc.history_table();
    }

    #[test]
    fn test_sell_close() {
        println!("test sell close");
        let code = "RB2005";

        let mut acc = QA_Account::new("RustT01B2_RBL8", "test", "admin",
                                      100000.0, false);
        acc.init_h(code);
        acc.buy_open(code, 10.0, "2020-01-20", 3500.0);
        assert_eq!(acc.get_volume_long(code), 10.0);
        acc.sell_close(code, 10.0, "2020-01-20", 3600.0);

        assert_eq!(acc.get_volume_long(code), 0.0);
        //println!("{:#?}", )
        acc.history_table();
    }
}