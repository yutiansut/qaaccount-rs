use tokio::runtime::Builder;
use tokio::sync::oneshot;
use tokio::task;
use std::rc::Rc;
use std::cell::RefCell;
use quantaxis_rs::qaaccount::QA_Account;
use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex};
use tokio::runtime;
use std::ops::DerefMut;

async fn some_computation() -> String {
    "represents the result of the computation".to_string()
}


#[tokio::main]
async fn main() {
    let code = "RB2005".to_string();
    let mut acc = QA_Account::new("RustT01B2_RBL8", "test", "admin", 100000.0, false, "real");
    acc.init_h(&code);
    let ac:Arc<Mutex<QA_Account>> = Arc::new(Mutex::new(acc));
    let mut ac1= ac.clone();
    let join  = task::spawn(async move {
        let mut acc_mut = ac1.lock().unwrap();
        let order = acc_mut.send_order_async(&code, 10.0, "2020-01-20 22:10:00", 2, 3500.0, "BUY_OPEN");
        // ac1.as_ref().borrow_mut().get_mut().unwrap().
        println!("下单完成");

    });
    let order = join.await.unwrap();

    //println!("{:#?}", acc.hold);
    println!("{:#?}", order);
}