use serde::{Deserialize, Serialize};

use crate::market_preset::{CodePreset, MarketPreset};
use crate::qaorder::QAOrder;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QA_Frozen {
    pub amount: f64,
    pub coeff: f64,
    pub money: f64,
}
impl QA_Frozen {
    pub fn reset(&mut self) {
        self.amount = 0.0;
        self.money = 0.0;
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QA_Postions {
    pub preset: CodePreset,
    pub code: String,
    pub instrument_id: String,
    pub user_id: String,
    pub portfolio_cookie: String,
    pub username: String,
    pub position_id: String,
    pub account_cookie: String,
    pub frozen: f64,
    pub name: String,
    pub spms_id: String,
    pub oms_id: String,
    pub market_type: String,
    pub exchange_id: String,
    pub lastupdatetime: String,
    //# 持仓量
    pub volume_long_today: f64,
    pub volume_long_his: f64,
    pub volume_short_today: f64,
    pub volume_short_his: f64,
    //# 平仓委托冻结(未成交)
    pub volume_long_frozen_today: f64,
    pub volume_long_frozen_his: f64,

    pub volume_short_frozen_today: f64,
    pub volume_short_frozen_his: f64,

    //# 保证金
    pub margin_long: f64,
    pub margin_short: f64,
    //# 持仓字段
    pub position_price_long: f64,
    pub position_cost_long: f64,
    pub position_price_short: f64,
    pub position_cost_short: f64,
    //# 平仓字段
    pub open_price_long: f64,
    pub open_cost_long: f64,
    pub open_price_short: f64,
    pub open_cost_short: f64,

    pub lastest_price: f64,
    pub lastest_datetime: String
}

impl QA_Postions{
    pub(crate) fn message(& self) {
        println!("{}", self.code.clone());
    }
    pub fn new(code:String, user_id: String,
                username: String, account_cookie: String,
                portfolio_cookie:String) -> Self {
        let mut preset: CodePreset = MarketPreset::new().get(code.as_ref());

        let pos = Self {
            preset,
            code: code.clone(),
            instrument_id: code.clone(),
            user_id,
            portfolio_cookie,
            username,
            position_id: "".to_string(),
            account_cookie,
            frozen: 0.0,
            name: "".to_string(),
            spms_id: "".to_string(),
            oms_id: "".to_string(),
            market_type: "".to_string(),
            exchange_id: "".to_string(),
            lastupdatetime: "".to_string(),
            volume_long_today: 0.0,
            volume_long_his: 0.0,

            volume_short_today: 0.0,
            volume_short_his: 0.0,

            volume_long_frozen_today: 0.0,
            volume_long_frozen_his: 0.0,

            volume_short_frozen_today: 0.0,
            volume_short_frozen_his: 0.0,

            margin_long: 0.0,
            margin_short: 0.0,

            position_price_long: 0.0,
            position_cost_long: 0.0,

            position_price_short: 0.0,
            position_cost_short: 0.0,

            open_price_long: 0.0,
            open_cost_long: 0.0,

            open_price_short: 0.0,
            open_cost_short: 0.0,
            lastest_price: 0.0,
            lastest_datetime: "".to_string()
        };
        pos
    }

    pub fn margin(&mut self) -> f64 {
        self.margin_long + self.margin_short
    }


    pub fn on_price_change(&mut self, price: f64, datetime: String) {
        // 当行情变化时候 要更新计算持仓
        self.lastest_price = price;
        self.lastest_datetime = datetime;
    }

    pub fn float_profit_long(&mut self) -> f64 {
        self.lastest_price * self.volume_long() * self.preset.unit_table as f64 - self.open_cost_long
    }

    pub fn float_profit_short(&mut self) -> f64 {
        self.open_cost_short - self.lastest_price * self.volume_short() * self.preset.unit_table as f64
    }


    pub fn float_profit(&mut self) -> f64 {
        self.float_profit_long() + self.float_profit_short()
    }


    pub fn position_profit_long(&mut self) -> f64 {
        self.lastest_price * self.volume_long() * self.preset.unit_table as f64 - self.position_cost_long
    }

    pub fn position_profit_short(&mut self) -> f64 {
        self.position_cost_short - self.lastest_price * self.volume_short() * self.preset.unit_table as f64
    }


    pub fn volume_long(&mut self) -> f64 {
        self.volume_long_today + self.volume_long_his + self.volume_long_frozen()
    }
    pub fn volume_short(&mut self) -> f64 {
        self.volume_short_his + self.volume_short_today + self.volume_short_frozen()
    }

    pub fn volume_long_frozen(&mut self) -> f64 {
        self.volume_long_frozen_his + self.volume_long_frozen_today
    }
    pub fn volume_short_frozen(&mut self) -> f64 {
        self.volume_short_frozen_his + self.volume_short_frozen_today
    }


    pub fn update_pos(&mut self, price: f64, amount: f64, towards: i32) -> (f64, f64) {
        let temp_cost = self.preset.calc_marketvalue(price, amount);
        let mut margin_value = temp_cost * self.preset.buy_frozen_coeff;
        self.lastest_price = price;
        //self.on_price_change(price.clone());
        let mut profit = 0.0;
        match towards {
            2 => {
                // buy open logic
                self.margin_long += margin_value;
                self.open_price_long = (self.open_price_long * self.volume_long() +
                    price * amount) / (self.volume_long() + amount);
                self.position_price_long = self.open_price_long;
                self.volume_long_today += amount;
                self.open_cost_long += temp_cost;
                self.position_cost_long += temp_cost;
            }
            -2 => {
                // sell open logic
                self.margin_short += margin_value;
                self.open_price_short = (self.open_price_short * self.volume_short() +
                    price * amount) / (self.volume_short() + amount);
                self.position_price_short = self.open_price_short;
                self.volume_short_today += amount;
                self.open_cost_short += temp_cost;
                self.position_cost_short += temp_cost;
            }
            3 => {
                //self.volume_short_today -= amount;
                // 有昨仓先平昨仓

                let volume_short = self.volume_short();
                self.position_cost_short = self.position_cost_short * (volume_short - amount) / volume_short;
                self.open_cost_short = self.open_cost_short * (volume_short - amount) / volume_short;

                self.volume_short_frozen_today -= amount;

                //println!("amount  {},position_price_short {}", amount, self.position_price_short);

                //self.preset.print();

                margin_value = -1.0 * (self.position_price_short * amount *
                    self.preset.sell_frozen_coeff *
                    self.preset.unit_table as f64);

                //println!("BUY CLOSE XX MV{:#?}", margin_value);

                profit = (self.position_price_short - price) * amount * self.preset.unit_table as f64;
                self.margin_short += margin_value;
            }
            -3 => {
                //self.volume_long_today -= amount;

                let volume_long = self.volume_long();
                self.position_cost_long = self.position_cost_long * (volume_long - amount) / volume_long;
                self.open_cost_long = self.open_cost_long *
                    (volume_long - amount) / volume_long;

                self.volume_long_frozen_today -= amount;
                margin_value = -1.0 * (self.position_price_long * amount * self.preset.unit_table as f64 *
                    self.preset.buy_frozen_coeff);
                profit = (price - self.position_price_long) *
                    amount * self.preset.unit_table as f64;
                self.margin_long += margin_value;
            }
            _ => {}
        }
        (margin_value, profit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        // create a new account
        let mut pos = QA_Postions::new("rb2005".to_string(), "test".to_string(), "test_username".to_string(),
                                       "test_accountcookie".to_string(), "test_portfolio".to_string());
        pos.message()
    }


    #[test]
    fn test_receivedeal() {
        // create a new account
        let mut pos = QA_Postions::new("rb2005".to_string(), "test".to_string(), "test_username".to_string(),
                                       "test_accountcookie".to_string(), "test_portfolio".to_string());
        pos.update_pos(3600.0, 10.0, 2); //buy open

        assert_eq!(10.0, pos.volume_long());
    }


    #[test]
    fn test_onpricechange() {
        // create a new account
        let mut pos = QA_Postions::new("rb2005".to_string(), "test".to_string(), "test_username".to_string(),
                                       "test_accountcookie".to_string(), "test_portfolio".to_string());
        pos.update_pos(3600.0, 10.0, 2); //buy open

        assert_eq!(10.0, pos.volume_long());
        pos.on_price_change(3605.0, "2020-02-20 09:55:00".to_string());
        println!("float profit{}", pos.float_profit());

        assert_eq!(500.0, pos.float_profit_long());
        assert_eq!(500.0, pos.float_profit());
        pos.on_price_change(3589.0, "2020-02-20 13:55:00".to_string());
        println!("float profit{}", pos.float_profit());
        assert_eq!(-1100.0, pos.float_profit_long());
        pos.update_pos(3585.0, 10.0, -2); //sell open

        assert_eq!(-1500.0, pos.float_profit_long());
        assert_eq!(0.0, pos.float_profit_short());

        pos.on_price_change(3605.0, "2020-02-20 09:55:00".to_string());
        println!("float profit{}", pos.float_profit());

        assert_eq!(-2000.0, pos.float_profit_short());
        assert_eq!(500.0, pos.float_profit_long());

        pos.on_price_change(3589.0, "2020-02-20 13:55:00".to_string());
        println!("float profit{}", pos.float_profit());

        assert_eq!(-400.0, pos.float_profit_short());
        assert_eq!(-1100.0, pos.float_profit_long());
    }
}