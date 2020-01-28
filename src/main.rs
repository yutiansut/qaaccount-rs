

pub mod qaaccount;
pub mod qafetch;
pub mod qaorder;



    fn main(){


    let mut acc  = qaaccount::QA_Account {
        cash: vec![],
        hold: vec![],
        account_cookie: "x".to_string(),
        portfolio_cookie: "x".to_string(),
        user_cookie: "x".to_string()
    };


    qaaccount::QA_Account::send_order(&mut acc,
        "rb2005", 10.0, "2020-01-28", 2, 3000.0, "order"
    );

    qaaccount::QA_Account::history(&mut acc);
}

