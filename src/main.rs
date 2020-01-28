//noinspection RsWrongLifetimeParametersNumber
// use serde;
// use serde_json::json;
// use serde::{Deserialize, Serialize};
// use serde_json::value::Value;
//#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QA_Postions {
    code: String,
    instrument_id: String,
    user_id: String,
    portfolio_cookie: String,
    username: String,
    position_id: String,
    account_cookie: String,
    frozen: f64,
    name: String,
    spms_id: String,
    oms_id: String,
    market_type: String,
    exchange_id: String,
    lastupdatetime: String,
    //# 持仓量
    volume_long_today: f64,
    volume_long_his: f64,
    volume_long: f64,
    volume_short_today: f64,
    volume_short_his: f64,
    volume_short: f64,
    //# 平仓委托冻结(未成交)
    volume_long_frozen_today: f64,
    volume_long_frozen_his: f64,
    volume_long_frozen: f64,
    volume_short_frozen_today: f64,
    volume_short_frozen_his: f64,
    volume_short_frozen: f64,
    //# 保证金
    margin_long: f64,
    margin_short: f64,
    margin: f64,
    //# 持仓字段
    position_price_long: f64,
    position_cost_long: f64,
    position_price_short: f64,
    position_cost_short: f64,
    //# 平仓字段
    open_price_long: f64,
    open_cost_long: f64,
    open_price_short: f64,
    open_cost_short: f64,

}

impl QA_Postions{
    fn message(& self) {
        println!("{}", self.code.clone());
    }
}

pub struct QA_Order {

}



pub struct QA_Account{
    cash: Vec<f64>,
    hold: Vec<QA_Postions>,
    account_cookie: String,
    portfolio_cookie: String,
    user_cookie: String,

}


impl QA_Account{
    pub fn history(&mut self){
        for item in self.hold.iter(){
            QA_Postions::message(item);
        }
    }
    pub fn send_order(
        &mut self,
        code: &str,
        amount: f64,
        time: &str,
        towards: i32,
        price:f64,
        order_id :&str
    ) {
        println!("{}", code);

        let pos = QA_Postions{
            code: code.to_string(),
            instrument_id: code.to_string(),
            user_id: self.account_cookie.clone(),
            portfolio_cookie: self.portfolio_cookie.clone(),
            username: "".to_string(),
            position_id: "".to_string(),
            account_cookie: self.account_cookie.clone(),
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
        self.hold.push(pos);

    }
}


fn main(){
    let mut acc  = QA_Account {
        cash: vec![],
        hold: vec![],
        account_cookie: "x".to_string(),
        portfolio_cookie: "x".to_string(),
        user_cookie: "x".to_string()
    };


    QA_Account::send_order(&mut acc,
        "rb2005", 10.0, "2020-01-28", 2, 3000.0, "order"
    );

    QA_Account::history(&mut acc);
}

