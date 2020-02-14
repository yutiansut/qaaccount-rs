use crate::market_preset::{CodePreset, MarketPreset};
use crate::qaorder::QAOrder;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
            open_cost_short: 0.0
        };
        pos
    }

    pub fn margin(&mut self) -> f64 {
        self.margin_long + self.margin_short
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

                println!("amount  {},position_price_short {}", amount, self.position_price_short);

                self.preset.print();

                margin_value = -1.0 * (self.position_price_short * amount *
                    self.preset.sell_frozen_coeff *
                    self.preset.unit_table as f64);

                println!("BUY CLOSE XX MV{:#?}", margin_value);

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

