use std::collections::HashMap;

use qifi_rs::account::Trade;

#[derive(Debug, Clone)]
struct QATradePair {
    open_datetiem: i64,
    close_datetime: i64,
    is_buy: bool,
    code: String,
    amount: f64,
    open_price: f64,
    close_price: f64,
    pnl_ration: f64,
    pnl_money: f64,
}

#[derive(Debug, Clone)]
struct Temp {
    amount: f64,
    direction: String,
    offset: String,
    datetime: i64,
    code: String,
    price: f64,
}

#[derive(Debug, Clone)]
struct QAPerformance {
    pair: Vec<QATradePair>,
    temp: HashMap<String, Vec<Temp>>,
}


impl QAPerformance {
    fn new() -> Self {
        let mut temp = HashMap::new();
        temp.insert("BUY".to_string(), vec![]);
        temp.insert("SELL".to_string(), vec![]);

        QAPerformance { pair: vec![], temp }
    }
    fn insert_trade(&mut self, trade: Trade) {
        match trade.offset.as_str() {
            "OPEN" => {
                let direction = trade.direction.as_str();
                let u = self.temp.get_mut(direction).unwrap();
                u.push(Temp {
                    amount: trade.volume.clone(),
                    direction: trade.direction.clone(),
                    offset: "OPEN".to_string(),
                    datetime: trade.trade_date_time.clone(),
                    code: trade.instrument_id.clone(),
                    price: trade.price.clone(),
                });
            }
            "CLOSE" => {
                let (raw_direction, is_buy) = match trade.direction.as_str() {
                    "BUY" => { ("SELL", false) }
                    "SELL" => { ("BUY", true) }
                    _ => { ("", false) }
                };
                let u = self.temp.get_mut(raw_direction).unwrap();
                println!("{:#?}", u);
                let f = u.get_mut(0).unwrap();
                if trade.volume > f.amount {
                    // close> raw ==> 注销继续loop
                    self.pair.push(QATradePair {
                        open_datetiem: f.datetime.clone(),
                        close_datetime: trade.trade_date_time.clone(),
                        is_buy,
                        code: f.code.clone(),
                        amount: f.amount.clone(),
                        open_price: f.price.clone(),
                        close_price: trade.price.clone(),
                        pnl_ration: 0.0,
                        pnl_money: 0.0,
                    });
                    let mut new_t = trade.clone();
                    new_t.volume -= f.amount;
                    self.insert_trade(new_t)
                } else if trade.volume < f.amount {
                    self.pair.push(QATradePair {
                        open_datetiem: f.datetime.clone(),
                        close_datetime: trade.trade_date_time.clone(),
                        is_buy,
                        code: f.code.clone(),
                        amount: trade.volume.clone(),
                        open_price: f.price.clone(),
                        close_price: trade.price.clone(),
                        pnl_ration: 0.0,
                        pnl_money: 0.0,
                    });
                    f.amount -= trade.volume.clone();
                    //u.insert(0, f.clone());
                } else {
                    self.pair.push(QATradePair {
                        open_datetiem: f.datetime.clone(),
                        close_datetime: trade.trade_date_time.clone(),
                        is_buy,
                        code: f.code.clone(),
                        amount: f.amount.clone(),
                        open_price: f.price.clone(),
                        close_price: trade.price.clone(),
                        pnl_ration: 0.0,
                        pnl_money: 0.0,
                    });
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::qaaccount::QA_Account;

    use super::*;

    #[test]
    fn test_to_qifi() {
        let code = "rb2005";
        let mut p = QAPerformance::new();
        let mut acc = QA_Account::new(
            "RustT01B2_RBL8",
            "test",
            "admin",
            10000000.0,
            false,
            "real",
        );
        acc.init_h(code);
        acc.sell_open(code, 10.0, "2020-01-20 09:30:22", 3500.0);
        acc.buy_open(code, 10.0, "2020-01-20 09:52:00", 3500.0);
        assert_eq!(acc.get_volume_short(code), 10.0);
        assert_eq!(acc.get_volume_long(code), 10.0);
        acc.buy_close(code, 10.0, "2020-01-20 10:22:00", 3600.0);
        acc.buy_open(code, 10.0, "2020-01-20 13:54:00", 3500.0);
        acc.sell_open(code, 10.0, "2020-01-20 13:55:00", 3510.0);
        acc.buy_close(code, 10.0, "2020-01-20 13:56:00", 3514.0);
        acc.sell_close(code, 10.0, "2020-01-20 14:52:00", 3620.0);
        println!("{:#?}", acc.dailytrades);
        for (_, i) in acc.dailytrades.iter_mut() {
            println!("{:#?}", i);
            p.insert_trade(i.to_owned());
        }
        println!("{:#?}", p.pair);
    }
}

