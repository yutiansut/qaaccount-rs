use std::borrow::Borrow;
use std::collections::HashMap;
use std::error::Error;
use std::io;

use csv;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::market_preset::{CodePreset, MarketPreset};
use crate::qaorder::QAOrder;
use crate::qaposition;
use crate::qaposition::{QA_Frozen, QA_Postions};
use crate::transaction;
use crate::transaction::QATransaction;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QAAccountSlice {
    pub datetime: String,
    pub cash: f64,
    pub accounts: account,
    pub events: HashMap<String, String>,
    pub positions: HashMap<String, QA_Postions>,
    pub frozen: HashMap<String, QA_Frozen>,
    pub trades: HashMap<String, QATransaction>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct account {
    user_id: String,
    // 用户号 兼容diff协议, ==> 实盘则为具体账户号
    currency: String,
    // 货币属性 兼容diff协议
    pre_balance: f64,
    // 上一个交易日的结算权益
    deposit: f64,
    // 今日转入资金
    withdraw: f64,
    // 今日转出资金
    WithdrawQuota: f64,
    // 当前可取字段(QIFI 独有)
    close_profit: f64,
    // 平仓盈亏
    commission: f64,
    // 手续费
    premium: f64,
    // 附加费
    static_balance: f64,
    // 静态权益(一般= pre_balance)
    position_profit: f64,
    // 持仓盈亏
    float_profit: f64,
    // 浮动盈亏
    balance: f64,
    // 当前权益
    margin: f64,
    // 保证金
    frozen_margin: f64,
    // 冻结保证金
    frozen_commission: f64,
    // 冻结手续费
    frozen_premium: f64,
    // 冻结附加费用
    available: f64,
    // 可用资金
    risk_ratio: f64,   // 风险度
}


#[warn(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct QA_Account {
    init_cash: f64,
    init_hold: HashMap<String, QA_Postions>,

    allow_t0: bool,
    allow_sellopen: bool,
    allow_margin: bool,

    auto_reload: bool,
    market_preset: MarketPreset,
    time: String,
    pub events: HashMap<String, String>,
    pub accounts: account,
    pub cash: Vec<f64>,
    pub money: f64,
    pub trades: HashMap<String, QATransaction>,
    pub hold: HashMap<String, QA_Postions>,
    pub frozen: HashMap<String, QA_Frozen>,
    pub dailyassets: HashMap<String, QAAccountSlice>,
    pub history: Vec<transaction::QATransaction>,
    pub account_cookie: String,
    pub portfolio_cookie: String,
    pub user_cookie: String,

    environment: String,

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
            time: "".to_string(),
            events: HashMap::new(),
            accounts: account {
                user_id: account_cookie.to_string(),
                currency: "CNY".to_string(),
                pre_balance: init_cash.clone(),
                deposit: 0.0,
                withdraw: 0.0,
                WithdrawQuota: init_cash.clone(),
                close_profit: 0.0,
                commission: 0.0,
                premium: 0.0,
                static_balance: init_cash.clone(),
                position_profit: 0.0,
                float_profit: 0.0,
                balance: init_cash.clone(),
                margin: 0.0,
                frozen_margin: 0.0,
                frozen_commission: 0.0,
                frozen_premium: 0.0,
                available: init_cash.clone(),
                risk_ratio: 0.0,
            },
            cash: vec![init_cash],
            money: init_cash,
            hold: HashMap::new(),
            trades: HashMap::new(),
            frozen: HashMap::new(),
            history: vec![],
            account_cookie: account_cookie.parse().unwrap(),
            portfolio_cookie: portfolio_cookie.parse().unwrap(),
            user_cookie: user_cookie.parse().unwrap(),
            environment: environment.to_string(),
            dailyassets: HashMap::new(),
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

    pub fn get_cash(&mut self) -> f64 {
        self.cash.last().unwrap().to_owned()
    }
    pub fn get_riskratio(&mut self) -> f64 {
        0.0
    }

    pub fn get_accontmessage(&mut self) -> account {
        account {
            user_id: self.account_cookie.clone(),
            currency: "CNY".to_string(),
            pre_balance: self.accounts.pre_balance,
            deposit: self.accounts.deposit,
            withdraw: self.accounts.withdraw,
            WithdrawQuota: self.accounts.WithdrawQuota,
            close_profit: self.accounts.close_profit,
            commission: self.accounts.commission,
            premium: self.accounts.premium,
            static_balance: self.accounts.static_balance,
            position_profit: self.get_positionprofit(),
            float_profit: self.get_floatprofit(),
            balance: self.get_balance(),
            margin: self.get_margin(),
            frozen_margin: 0.0,
            frozen_commission: 0.0,
            frozen_premium: 0.0,
            available: self.money,
            risk_ratio: self.get_riskratio(),
        }
    }
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

    /// frozen & margin
    pub fn get_frozen(&mut self, code: &str) -> f64 {
        self.get_position(code).unwrap().frozen
    }
    pub fn get_margin(&mut self) -> f64 {
        let mut margin = 0.0;
        for pos in self.hold.values_mut() {
            margin += pos.margin();
        }
        margin
    }


    /// profit
    pub fn get_floatprofit(&mut self) -> f64 {
        let mut fp = 0.0;
        for pos in self.hold.values_mut() {
            fp += pos.float_profit();
        }
        fp
    }
    pub fn get_positionprofit(&mut self) -> f64 {
        let mut pp = 0.0;
        for pos in self.hold.values_mut() {
            pp += pos.float_profit();
        }
        pp
    }

    /// balance
    pub fn get_balance(&mut self) -> f64 {
        let fp = self.get_floatprofit();
        //println!("{} {} {} {} {}", self.accounts.static_balance, self.accounts.deposit, self.accounts.withdraw, fp, self.accounts.close_profit);
        self.accounts.static_balance + self.accounts.deposit - self.accounts.withdraw + fp + self.accounts.close_profit
    }


    pub fn settle(&mut self) {
        self.dailyassets.insert(self.time.clone(), QAAccountSlice {
            datetime: self.time.clone(),
            cash: self.money.clone(),
            accounts: self.accounts.clone(),
            events: self.events.clone(),
            positions: self.hold.clone(),
            frozen: self.frozen.clone(),
            trades: self.trades.clone(),
        });
        self.trades = HashMap::new();
        self.events = HashMap::new();

        // init the next day cash
        let balance_settle = self.accounts.pre_balance + self.accounts.close_profit;
        self.accounts = account {
            user_id: self.account_cookie.to_string(),
            currency: "CNY".to_string(),
            pre_balance: balance_settle.clone(),
            deposit: 0.0,
            withdraw: 0.0,
            WithdrawQuota: balance_settle.clone(),
            close_profit: 0.0,
            commission: 0.0,
            premium: 0.0,
            static_balance: balance_settle.clone(),
            position_profit: 0.0,
            float_profit: 0.0,
            balance: balance_settle.clone(),
            margin: self.accounts.margin,
            frozen_margin: 0.0,
            frozen_commission: 0.0,
            frozen_premium: 0.0,
            available: balance_settle.clone(),
            risk_ratio: self.get_riskratio(),
        }
    }


    pub fn get_codeSubscribed(&mut self) -> Vec<String> {
        // if a QAAccount trades a packages, then it need the get_codeSubscribed to update the price
        // some examples like below
        // let codes = account.get_codeSubscribed();
        // for code in codes.iter():
        //     acc.on_price_change(code, price.get(code), datetime)
        let mut codeSub = vec![];
        for key in self.hold.keys() {
            codeSub.push(key.to_string())
        }
        codeSub
    }

    pub fn get_slice(&mut self) -> QAAccountSlice {
        // get a realtime slice of account
        // this can be save to database

        QAAccountSlice {
            datetime: self.time.clone(),
            cash: self.money.clone(),
            accounts: self.accounts.clone(),
            events: self.events.clone(),
            positions: self.hold.clone(),
            frozen: self.frozen.clone(),
            trades: self.trades.clone(),
        }
    }

    /// history about

    pub fn history_table(&self) {
        for item in self.history.iter() {
            println!("{:?}", transaction::QATransaction::to_json(item));
        }
    }


    pub fn to_csv(&self) -> Result<(), Box<dyn Error>> {
        let mut wtr = csv::Writer::from_path(format!("{}.csv", self.account_cookie)).unwrap();
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

                let frozen = coeff * amount;
//                println!("OPEN FROZEN{:#?}", frozen);
//                println!("ORDER ID {:#?}", order_id);
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
            match self.environment.as_ref() {
                "backtest" => {
                    self.receive_deal(code.parse().unwrap(), amount, price, time.parse().unwrap(),
                                      order_id.clone(), order_id.clone(), order_id.clone(),
                                      towards);
                }
                "real" => {
                    self.events.insert(self.time.clone(), "order insert".to_string());
                }
                _ => {
                    self.events.insert(self.time.clone(), "order insert".to_string());
                }
            }
            Ok(order.clone())
        } else {
            Err(())
        }

    }


    pub fn on_price_change(&mut self, code: String, price: f64, datetime: String) {
        // 当行情变化时候 要更新计算持仓
        let pos = self.get_position(code.as_ref()).unwrap();
        pos.on_price_change(price, datetime);
    }


    fn receive_deal(&mut self, code: String, amount: f64, price: f64, datetime: String,
                    order_id: String, trade_id: String, realorder_id: String, towards: i32,
    ) {
        self.time = datetime.clone();
        if self.frozen.contains_key(&order_id) {
            let frozen = self.frozen.get_mut(&order_id).unwrap();
            self.money += frozen.money;
            self.frozen.remove(&order_id);

            // self.frozen.insert(order_id.clone(), QA_Frozen {
            //     amount: 0.0,
            //     coeff: 0.0,
            //     money: 0.0,
            // });
        } else {
            println!("ERROR NO THAT ORDER {}", order_id)
        }


        let qapos = self.get_position(code.as_ref()).unwrap();
        qapos.on_price_change(price.clone(), datetime.clone());

        let (margin, close_profit) = qapos.update_pos(price, amount, towards);

        //println!("MARGIN RELEASE {:#?}", margin.clone());
        //println!("CLOSE PROFIT RELEASE {:#?}", close_profit.clone());
        self.money -= (margin - close_profit);
        self.accounts.close_profit += close_profit;
        self.cash.push(self.money);

        let transaction = transaction::QATransaction {
            code,
            amount,
            price,
            datetime,
            order_id,
            trade_id: trade_id.clone(),
            realorder_id,
            account_cookie: self.account_cookie.clone(),
            commission: 0.0,
            tax: 0.0,
            message: "".to_string(),
            frozen: 0.0,
            direction: towards,
        };
        self.trades.insert(trade_id, transaction.clone());
        self.history.push(transaction);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        // create a new account
        let mut acc = QA_Account::new("RustT01B2_RBL8", "test", "admin",
                                      100000.0, false, "backtest");
        acc.history_table();
    }
    #[test]
    fn test_pos() {
        // test get position function
        let mut acc = QA_Account::new("RustT01B2_RBL8", "test", "admin",
                                      100000.0, false, "backtest");
        acc.init_h("RB2005");
        acc.get_position("RB2005");
    }

    #[test]
    fn test_init_h() {
        // test init a position function
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
        println!("LATEST MONEY {:#?}", acc.money);
        println!("CLOSE PROFIT {:#?}", acc.accounts.close_profit);
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
        println!("LATEST MONEY {:#?}", acc.money);
        println!("CLOSE PROFIT {:#?}", acc.accounts.close_profit);

        acc.history_table();
    }

    #[test]
    fn test_accountSlice() {
        println!("test account slice");
        let code = "RB2005";

        let mut acc = QA_Account::new("RustT01B2_RBL8", "test", "admin",
                                      100000.0, false, "backtest");
        acc.init_h(code);
        acc.buy_open(code, 10.0, "2020-01-20", 3500.0);
        let slice = acc.get_slice();
        println!("account Slice  {:#?}", slice);
        assert_eq!(acc.get_volume_long(code), 10.0);
        acc.sell_close(code, 10.0, "2020-01-20", 3600.0);

        assert_eq!(acc.get_volume_long(code), 0.0);
        //println!("{:#?}", )
        println!("LATEST MONEY {:#?}", acc.money);
        println!("CLOSE PROFIT {:#?}", acc.accounts.close_profit);

        let slice = acc.get_slice();

        println!("account Slice  {:#?}", slice);

        acc.history_table();
    }

    #[test]
    fn test_getaccountmessage() {
        println!("test account slice");
        let code = "RB2005";

        let mut acc = QA_Account::new("RustT01B2_RBL8", "test", "admin",
                                      100000.0, false, "backtest");
        acc.init_h(code);
        acc.buy_open(code, 10.0, "2020-01-20", 3500.0);

        let slice = acc.get_accontmessage();
        println!("account Slice  {:#?}", slice);
        assert_eq!(acc.get_volume_long(code), 10.0);
        acc.sell_close(code, 10.0, "2020-01-20", 3600.0);

        assert_eq!(acc.get_volume_long(code), 0.0);
        //println!("{:#?}", )
        println!("LATEST MONEY {:#?}", acc.money);
        println!("CLOSE PROFIT {:#?}", acc.accounts.close_profit);

        let slice = acc.get_accontmessage();

        println!("account Slice  {:#?}", slice);

        acc.history_table();
    }


    #[test]
    fn test_to_csv() {
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
        acc.to_csv();
        //acc.history_table();
    }
}