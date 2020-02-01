

use crate::qaorder;
use crate::transaction;
use std::collections::HashMap;
use crate::qaorder::QA_Postions;

#[warn(non_camel_case_types)]
pub struct QA_Account{
    pub cash: Vec<f64>,
    pub hold: HashMap<String, QA_Postions>,
    pub history: Vec<transaction::QATransaction>,
    pub account_cookie: String,
    pub portfolio_cookie: String,
    pub user_cookie: String,
}

impl QA_Account{

    pub fn new() -> Self {
        let acc  = Self{
            cash: vec![],
            hold: HashMap::new(),
            history: vec![],
            account_cookie: "".to_string(),
            portfolio_cookie: "".to_string(),
            user_cookie: "".to_string(),

        };
        acc
    }
    pub fn init_h (&mut self, code:&str){
        let code:String = code.parse().unwrap();
        self.hold.insert(code.clone(), QA_Postions::new(code.clone(), self.account_cookie.clone(),
                                                        self.account_cookie.clone(), self.account_cookie.clone(),
                                                        self.portfolio_cookie.clone()));
    }
    pub fn get_position(&mut self, code: &str) -> Option<&mut QA_Postions> {

        let pos = self.hold.get_mut(code);
        pos

    }
    pub fn get_position_long(&mut self, code: &str) -> f64 {
        self.get_position(code).unwrap().volume_long_today
    }
    pub fn get_position_short(&mut self, code: &str) -> f64 {
        self.get_position(code).unwrap().volume_short_today
    }
    pub fn history_table(& self){
        for item in self.history.iter(){
            println!("{:?}",transaction::QATransaction::to_json(item));
        }
    }
    pub fn buy(
        &mut self,
        code: &str,
        amount: f64,
        time: &str,
        price:f64
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
        price:f64
    ) {
        self.send_order(code, amount, time, -3, price, "SELL_CLOSE");
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
        println!("{} - {}", code, towards);
        let mut pos = self.get_position(code).unwrap();
        pos.update_pos( price, amount, towards);



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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mut acc = QA_Account::new();
        acc.history_table();
    }
    #[test]
    fn test_pos(){
        let mut acc = QA_Account::new();
        acc.init_h("RB2005");
        acc.get_position("RB2005");
    }
    #[test]
    fn test_init_h(){
        let mut acc = QA_Account::new();

        acc.init_h("RB2005");
        println!("{:#?}",acc.get_position("RB2005").unwrap().message());
    }
}