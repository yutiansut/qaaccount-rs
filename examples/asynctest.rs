use tokio::runtime::Builder;
use tokio::sync::oneshot;
use tokio::task;

use quantaxis_rs::qaaccount::QA_Account;

async fn some_computation() -> String {
    "represents the result of the computation".to_string()
}

#[tokio::main]
async fn main() {
    let code = "RB2005";
    let mut acc = QA_Account::new("RustT01B2_RBL8", "test", "admin", 100000.0, false, "real");
    acc.init_h(code);

    acc.send_order_async(code, 10.0, "2020-01-20 22:10:00", 2, 3500.0, "BUY_OPEN").await;

    println!("{:?}", acc.hold);
}