import QUANTAXIS as QA
import pandas as pd

user = QA.QA_User(username='admin', password='admin')
port = user.new_portfolio('rust')
debug_acc = port.new_account('Rust_T01B2_RBL8_2019', market_type=QA.MARKET_TYPE.FUTURE_CN, init_cash=1000000)
trade_his = pd.read_csv('acc.csv')
for _, item in trade_his.iterrows():

    res = debug_acc.receive_simpledeal(code=item['code'], trade_price=item['price'],
                                       trade_amount=abs(item['amount']),
                                       trade_towards=item['direction'],
                                       trade_time=item['datetime'])
    #print(debug_acc.cash_available)

print(debug_acc.history_table)
debug_acc.save()
Risk = QA.QA_Risk(debug_acc)
Risk.save()
