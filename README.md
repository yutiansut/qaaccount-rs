# QUANTAXIS-RS 这是你没有见过的QUANTAXIS的全新版本

本项目已经迁移至 QUANTAXIS 主项目中  :  github.com/quantaxis/quantaxis

![Rust](https://github.com/yutiansut/quantaxis-rs/workflows/Rust/badge.svg)

## 特性
- 完整的quantaxis Account/Data/Indicator功能 支持快速的回测
- 单票分钟线2年回测在500ms
- 单指标计算在70ns
- 完整的测试工具，对于每个函数都有完整对应的测试
- 完整的benchmark工具链

## 兼容性

- 兼容python版本的QUANTAXIS回测
- 兼容基于QIFI协议的所有项目


power by yutiansut/somewheve

2020


### update MARKET_PRESET
```python

import QUANTAXIS as QA


mp = QA.QAARP.MARKET_PRESET()
for k,i in mp.table.items():
    print(f"""market_preset.insert(
            "{k}".to_string(),
            CodePreset {{
                name: "{i['name']}".to_string(),
                unit_table: {int(i['unit_table'])},
                price_tick: {float(i['price_tick'])},
                buy_frozen_coeff: {i['buy_frozen_coeff']},
                sell_frozen_coeff:{i['sell_frozen_coeff']},
                exchange: "{i['exchange']}".to_string(),
                commission_coeff_peramount: {float(i['commission_coeff_peramount'])},
                commission_coeff_pervol:{float(i['commission_coeff_pervol'])},
                commission_coeff_today_peramount: {float(i['commission_coeff_today_peramount'])},
                commission_coeff_today_pervol: {float(i['commission_coeff_today_pervol'])},
            }},
        );""")

```
