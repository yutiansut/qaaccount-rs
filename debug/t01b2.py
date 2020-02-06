import QUANTAXIS as QA
import numpy as np
from QAStrategy import QAStrategyCTABase


class RBT01(QAStrategyCTABase):
    count1=0
    HAE=0
    LAE=0

    def calc_HHV(self,market_data, n):
        try:
            ind = QA.HHV(market_data.iloc[-n-1:]['high'],n)
            return ind.iloc[-2]
        except Exception as e:
            print(e)
            return np.nan

    def calc_LLV(self,market_data, n):
        try:
            ind = QA.LLV(market_data.iloc[-n-1:]['low'],n)
            return ind.iloc[-2]
        except Exception as e:
            print(e)
            return np.nan

    def on_bar(self, bar):
        #print(bar)
        mp = bar['close']* self.acc.market_preset.get_frozen(self.code)* self.acc.market_preset.get_unit(self.code)
        lots = int(300000/mp) -1

        priceoffset = 1
        lossP = 1.3
        K1 = 20
        K2 = 20
        n1 = 30
        TrailingStart1 = 90
        TrailingStop1 = 10

        bar_id = self.bar_id
        market_data = self.market_data
        market_datetime = self.market_datetime


        if len(market_data) > 2:
            print(market_data.index[-1])
            bar['lastclose'] =market_data.iloc[-2]['close']
            bar['lasthigh'] = market_data.iloc[-2]['high']
            bar['lastlow'] = market_data.iloc[-2]['low']
            #             timec = (datetime.datetime.strptime(market_data[-1]['datetime'],'%Y-%m-%d %H:%M:%S')-datetime.datetime.strptime(market_data[-2]['datetime'],'%Y-%m-%d %H:%M:%S')).total_seconds()/3600
            hour = int(str(market_datetime[-1])[11:13])

            HHV = self.calc_HHV(market_data, K1)
            print("HHV",HHV)
            LLV = self.calc_LLV(market_data, K2)
            print("LLV", LLV)
            #print(1)
            if self.positions.volume_long > 0 or self.positions.volume_short > 0:
                if bar_id-self.count1 == 1:
                    self.HAE = bar['lasthigh']
                    self.LAE = bar['lastlow']
                elif bar_id-self.count1 > 1:
                    self.HAE = max(bar['lasthigh'],self.HAE)
                    self.LAE = min(bar['lastlow'],self.LAE)

            CrossOver = bar['high'] > HHV and bar['lasthigh'] < HHV

            print("CrossOVer", CrossOver)
            CrossUnder = bar['low'] < LLV and bar['lastlow'] > LLV
            print("CrossUnder", CrossOver)
            #print(3)
            MA = QA.MA(market_data.iloc[-n1-5:-1]['open'],n1)

            print("MA", MA)


            cond1 = MA.iloc[-1]>MA.iloc[-2] and MA.iloc[-2]>MA.iloc[-3] and MA.iloc[-3]>MA.iloc[-4] and MA.iloc[-4]>MA.iloc[-5]
            cond2 = MA.iloc[-1]<MA.iloc[-2] and MA.iloc[-2]<MA.iloc[-3] and MA.iloc[-3]<MA.iloc[-4] and MA.iloc[-4]<MA.iloc[-5]
            #print(4)
            print("COND1",cond1)
            print("COND2", cond2)
            #-----------------------------------------------------------------------------------------------------------------------------------------------
            if self.positions.volume_long == 0 and self.positions.volume_short == 0 and hour >= 9 and hour <= 15:

                if CrossOver and cond1:
                    self.send_order('BUY', 'OPEN', price=max(bar['open'], HHV) + priceoffset, volume= lots)
                    self.count1 = bar_id
                    self.HAE=0
                    self.LAE=0
                    print('BUY_OPEN')

                if CrossUnder and cond2:
                    self.send_order('SELL', 'OPEN', price=min(bar['open'], LLV) - priceoffset, volume= lots)
                    self.count1 = bar_id
                    self.HAE=0
                    self.LAE=0
                    print('SELL_OPEN')
            #-----------------------------------------------------------------------------------------------------------------------------------------------

            elif self.positions.volume_long > 0 and self.positions.volume_short == 0:

                Stopline = round(self.positions.open_price_long*(100-lossP)/100,0)
                print('SELLCLOSE STOPLINE', Stopline)
                print('HAE', self.HAE)
                print('openprice', self.positions.open_price_long)
                print(self.HAE >= self.positions.open_price_long*(1+TrailingStart1/1000))
                if (self.HAE >= self.positions.open_price_long*(1+TrailingStart1/1000) and bar_id-self.count1 >= 1):
                    Stopline = self.HAE*(1-TrailingStop1/1000)
                    print('SELLCLOSE STOPLINE222', Stopline)

                if CrossUnder and cond2:
                    self.send_order('SELL', 'CLOSE', price=min(bar['open'], LLV) - priceoffset, volume= self.positions.volume_long)
                    print('SELL_CLOSE')

                elif bar['low'] <= Stopline:
                    self.send_order('SELL', 'CLOSE', price=min(bar['open'], Stopline) - priceoffset, volume= self.positions.volume_long)
                    print('SELL_CLOSE_STOPLOSS')
            #-----------------------------------------------------------------------------------------------------------------------------------------------
            elif self.positions.volume_long == 0 and self.positions.volume_short > 0:

                Stopline = round(self.positions.open_price_short*(100+lossP)/100,0)
                if (self.LAE <= self.positions.open_price_short*(1-TrailingStart1/1000) and bar_id-self.count1 >= 1):
                    Stopline = self.LAE*(1+TrailingStop1/1000)
                if CrossOver and cond1:
                    self.send_order('BUY', 'CLOSE', price=max(bar['open'],  HHV) + priceoffset, volume= self.positions.volume_short)
                    print('BUY_CLOSE')

                elif bar['high'] >= Stopline:
                    self.send_order('BUY', 'CLOSE', price=max(bar['open'], Stopline) + priceoffset, volume= self.positions.volume_short)
                    print('BUY_CLOSE_STOPLOSS')
            #-----------------------------------------------------------------------------------------------------------------------------------------------
            if self.positions.volume_long == 0 and self.positions.volume_short == 0:
                self.count1 = 0
                self.HAE = 0
                self.LAE = 0
# "ru2005", "sc2002"

running ={}
#'TAL8', 'ML8', 'RBL8', 'IL8', 'JL8', 'NIL8', 'ZNL8', 'CFL8', 'PL8', 'AL8', 'SML8', 'MAL8', 'SRL8', 'CUL8', 'VL8', 'HCL8', 'LL8', 'SFL8',

#'URL8', 'RRL8', 'WHL8', 'APL8', 'ICL8', 'CJL8', 'FGL8', 'RML8', 'SSL8', 'CYL8', 'SPL8', 'CL8', 'JML8', 'BL8'
#for item in [ 'BUL8', 'IHL8', 'AUL8', 'PPL8', 'AGL8', 'YL8', 'ZCL8', 'IFL8', 'OIL8', 'JDL8', 'RUL8', 'PBL8', 'EGL8', 'CSL8', 'ALL8', 'EBL8', ]:
for item in ['RBL8']:
    running[item] = {}
    for frequence in ['15min']:
        running[item][frequence] = RBT01(code=item, frequence=frequence,portfolio= 'debugx',
                                         strategy_id='DETx5_{}_{}'.format(item, frequence), send_wx=True,start='2019-01-01', end= '2020-01-22')

        running[item][frequence].run_backtest()
