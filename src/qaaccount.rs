

use crate::qaorder;
use crate::transaction;


pub struct QA_Account{
    pub cash: Vec<f64>,
    pub hold: Vec<qaorder::QA_Postions>,
    pub history: Vec<transaction::QATransaction>,
    pub account_cookie: String,
    pub portfolio_cookie: String,
    pub user_cookie: String,

}


impl QA_Account{
    pub fn positions(&mut self){
        for item in self.hold.iter(){
            qaorder::QA_Postions::message(item);
        }
    }
    pub fn history_table(&mut self){
        for item in self.history.iter(){
            println!("{:?}",transaction::QATransaction::to_json(item));
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
        //println!("{}", code);

        let pos = qaorder::QA_Postions{
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
        self.history.push(transaction::QATransaction{
            code: code.to_string(),
            amount,
            price,
            datetime: time.to_string(),
            order_id: order_id.to_string(),
            trade_id: order_id.to_string(),
            realorder_id: order_id.to_string(),
            account_cookie: self.account_cookie.to_string(),
            commission: 0.0,
            tax: 0.0,
            message: "".to_string(),
            frozen: 0.0,
            direction: towards
        })

    }
}

