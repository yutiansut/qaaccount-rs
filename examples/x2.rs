use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use tokio::runtime;
use tokio::runtime::Builder;
use tokio::sync::oneshot;
use tokio::task;

use quantaxis_rs::qaaccount::QA_Account;

async fn some_computation() -> String {
    "represents the result of the computation".to_string()
}


#[tokio::main]
async fn main() {
    let code = "RB2005".to_string();
    let mut acc = QA_Account::new("RustT01B2_RBL8", "test", "admin", 100000.0, false, "real");
    let mut acc2 = QA_Account::new("RustT01B2_RBL8", "test", "admin", 100000.0, false, "real");
    let c = code.clone();
    acc.init_h(&code);
    acc2.init_h(&code);
    let join = task::spawn(async {
        let order = acc.send_order_async(&code, 10.0, "2020-01-20 22:10:00", 2, 3500.0, "BUY_OPEN").await;
        acc.settle();
        println!("ok2");
        println!("order: {:?}", order);
        (acc, code)
    });
    let join2 = task::spawn(async {
        let order = acc2.send_order_async(&c, 10.0, "2020-01-20 22:10:00", 2, 3500.0, "BUY_OPEN").await;
        acc2.settle();
        println!("2");
        println!("order: {:?}", order);
        (acc2, c)
    });

    let (acc, code) = join.await.unwrap();
    let (acc2, code2) = join2.await.unwrap();
}