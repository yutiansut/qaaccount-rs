use serde_json::to_string;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct QATransaction{

    pub code: String,
    pub amount: f64,
    pub price: f64,
    pub datetime: String,
    pub order_id: String,
    pub trade_id: String,
    pub realorder_id: String,
    pub account_cookie: String,
    pub commission: f64,
    pub tax: f64,
    pub message: String,
    pub frozen: f64,
    pub direction: i32
}

impl QATransaction{
    pub fn to_json(&self) -> String{
        let jdata= serde_json::to_string(&self).unwrap();
        jdata
    }
}