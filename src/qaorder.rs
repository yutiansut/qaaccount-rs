use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct QAOrder {
    account_cookie: String,
    user_id: String,
    instrument_id: String,
    towards: i32,
    exchange_id: String,
    order_time: String,
    volume: f64,
    price: f64,
    order_id: String,
    seqno: String,
    direction: String,
    offset: String,
    volume_orign: f64,
    price_type: String,
    limit_price: f64,
    time_condition: String,
    volume_condition: String,
    insert_date_time: String,
    exchange_order_id: String,
    status: i32,
    volume_left: f64,
    last_msg: String,
}

impl QAOrder {
    pub fn new(account: String, code: String, towards: i32, exchange_id: String, order_time: String,
               volume: f64, price: f64, order_id: String) -> Self {
        let mut direction = "BUY".to_string();
        let mut offset = "OPEN".to_string();

        match towards {
            1 | 2 => {}
            -1 => {
                direction = "SELL".to_string();
            }
            -2 => {
                direction = "SELL".to_string();
            }
            3 => {
                offset = "CLOSE".to_string();
            }
            -3 => {
                direction = "SELL".to_string();
                offset = "CLOSE".to_string();
            }
            _ => {}
        }


        Self {
            account_cookie: account.clone(),
            user_id: account.clone(),
            instrument_id: code.clone(),
            towards,
            exchange_id,
            order_time,
            volume,
            price,
            order_id,
            seqno: "".to_string(),
            direction,
            offset,
            volume_orign: 0.0,
            price_type: "LIMIT".to_string(),
            limit_price: price,
            time_condition: "AND".to_string(),
            volume_condition: "GFD".to_string(),
            insert_date_time: "".to_string(),
            exchange_order_id: Uuid::new_v4().to_string(),
            status: 100,
            volume_left: volume,
            last_msg: "".to_string(),
        }
    }
}