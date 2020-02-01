
pub struct QA_Postions {
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
    pub volume_long: f64,
    pub volume_short_today: f64,
    pub volume_short_his: f64,
    pub volume_short: f64,
    //# 平仓委托冻结(未成交)
    pub volume_long_frozen_today: f64,
    pub volume_long_frozen_his: f64,
    pub volume_long_frozen: f64,
    pub volume_short_frozen_today: f64,
    pub volume_short_frozen_his: f64,
    pub volume_short_frozen: f64,
    //# 保证金
    pub margin_long: f64,
    pub margin_short: f64,
    pub margin: f64,
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
                portfolio_cookie:String) -> Self{
        let pos = Self{
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
            volume_long: 0.0,
            volume_short_today: 0.0,
            volume_short_his: 0.0,
            volume_short: 0.0,
            volume_long_frozen_today: 0.0,
            volume_long_frozen_his: 0.0,
            volume_long_frozen: 0.0,
            volume_short_frozen_today: 0.0,
            volume_short_frozen_his: 0.0,
            volume_short_frozen: 0.0,
            margin_long: 0.0,
            margin_short: 0.0,
            margin: 0.0,
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

    pub fn update_pos(&mut self, price:f64, amount:f64, towards:i32){

        match towards {
            2 => {
                // buy open logic
                self.open_price_long = (self.open_price_long * self.volume_long_today + price* amount)/ (self.volume_long_today+ amount);
                self.position_price_long = self.open_price_long;
                self.volume_long_today += amount;

            }
            -2 => {
                // sell open logic
                self.open_price_short = (self.open_price_short * self.volume_short_today + price* amount)/ (self.volume_short_today + amount);
                self.position_price_short = self.open_price_short;
                self.volume_short_today += amount;
            }
            3 => {
                self.volume_short_today -= amount;
            }
            -3 => {
                self.volume_long_today -= amount;
            }
            _ => {}
        }


    }
}




pub struct QA_Order {

}


