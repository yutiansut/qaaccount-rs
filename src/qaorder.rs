
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
}

pub struct QA_Order {

}


