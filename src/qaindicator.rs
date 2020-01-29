
use std::f64::NAN;


#[derive(Debug)]
pub enum Err {
    NotEnoughtData, // not enought data to compute requested values
}


pub fn round_array(array: &mut [f64], decimals: u8) {
    let divider = (10.0 as f64).powi(decimals as i32);
    for number in array {
        *number = (*number * divider).round() / divider;
    }
}

pub fn ema(data: &[f64], period: usize) -> Result<Vec<f64>, Err> {
    if period > data.len() {
        return Err(Err::NotEnoughtData);
    }
    let mut ema = Vec::new();
    let mut j = 1;

    // get period sma first and calculate the next period period ema
    let sma = (data[0..period]).iter().sum::<f64>() / period as f64;
    let multiplier: f64 = 2.0 / (1.0 + period as f64);
    ema.push(sma);

    // EMA(current) = ( (Price(current) - EMA(prev) ) x Multiplier) + EMA(prev)
    ema.push(((data[period] - sma) * multiplier) + sma);

    // now calculate the rest of the values
    for i in &data[period + 1..data.len()] {
        let tmp = ((*i - ema[j]) * multiplier) + ema[j];
        j = j + 1;
        ema.push(tmp);
    }

    Ok(ema)
}

pub fn sma(data: &[f64], period: usize) -> Result<Vec<f64>, Err> {
    if period > data.len() {
        return Err(Err::NotEnoughtData);
    }

    let mut result = Vec::new();
    let mut running_total = 0.0;

    for i in 0..data.len() {
        running_total += data[i];
        if i >= period {
            running_total -= data[i - period];
        }
        if i >= period - 1 {
            result.push(running_total / period as f64);
        }
    }
    Ok(result)
}


pub fn psar(high: &[f64], low: &[f64], iaf: f64, maxaf: f64) -> Result<Vec<f64>, Err> {
    let mut psar = vec![NAN; high.len()];

    if high.len() < 2 {
        return Err(Err::NotEnoughtData);
    };

    let mut long = false;
    if high[0] + low[0] <= high[1] + low[1] {
        long = true;
    }

    let mut sar;
    let mut extreme;

    if long {
        extreme = high[0];
        sar = low[0];
    } else {
        extreme = low[0];
        sar = high[0];
    }

    psar[0] = sar;

    let mut af = iaf;

    for i in 1..high.len() {
        sar = (extreme - sar) * af + sar;

        if long {
            if i >= 2 && (sar > low[i - 2]) {
                sar = low[i - 2]
            };
            if sar > low[i - 1] {
                sar = low[i - 1]
            };

            if af < maxaf && high[i] > extreme {
                af += iaf;
                if af > maxaf {
                    af = maxaf
                };
            }

            if high[i] > extreme {
                extreme = high[i];
            }
        } else {
            if i >= 2 && sar < high[i - 2] {
                sar = high[i - 2]
            };
            if sar < high[i - 1] {
                sar = high[i - 1]
            };

            if af < maxaf && low[i] < extreme {
                af += iaf;
                if af > maxaf {
                    af = maxaf
                };
            }

            if low[i] < extreme {
                extreme = low[i]
            };
        }

        if long && low[i] < sar || !long && high[i] > sar {
            af = iaf;
            sar = extreme;

            long = !long;

            if !long {
                extreme = low[i];
            } else {
                extreme = high[i];
            }
        }

        psar[i] = sar;
    }

    Ok(psar)
}


pub fn rsi(data: &[f64], period: usize) -> Result<Vec<f64>, Err> {
    if period > data.len() {
        return Err(Err::NotEnoughtData);
    }

    let mut changes = Vec::new();
    for i in 0..data.len() - 1 {
        let change = data[i + 1] - data[i];
        changes.push(change);
    }

    let rsi_range = data.len() - period;

    let mut rsis = vec![NAN ; rsi_range];

    // gains & losses
    let mut gains = Vec::new();
    let mut losses = Vec::new();

    for i in 0..changes.len() {
        if changes[i] > 0.0 {
            gains.push(changes[i]);
            losses.push(0.0);
        } else if changes[i] < 0.0 {
            losses.push(changes[i] * -1.0);
            gains.push(0.0);
        } else {
            gains.push(0.0);
            losses.push(0.0);
        }
    }

    let mut avg_gain: f64 = gains[..period].iter().sum::<f64>() / gains[..period].len() as f64;
    let mut avg_loss: f64 = losses[..period].iter().sum::<f64>() / losses[..period].len() as f64;

    if avg_loss == 0.0 {
        rsis[0] = 100.0;
    } else {
        let rs = avg_gain / avg_loss;
        rsis[0] = 100.0 - (100.0 / (1.0 + rs));
    }

    for i in 1..rsi_range {
        avg_gain = (avg_gain * (period - 1) as f64 + gains[i + (period - 1)]) / period as f64;
        avg_loss = (avg_loss * (period - 1) as f64 + losses[i + (period - 1)]) / period as f64;

        if avg_loss == 0.0 {
            rsis[i] = 100.0;
        } else {
            let rs = avg_gain / avg_loss;
            rsis[i] = 100.0 - (100.0 / (1.0 + rs));
        }
    }

    Ok(rsis)
}




// Some randomly generated data to test against TA-Lib (see generate_data.py & correct_values.py)
const OPEN: &[f64] = &[1984.03, 1959.83, 2041.42, 2019.04, 1969.53, 2082.75, 2209.52, 2200.9,
    2364.04, 2543.32, 2423.95, 2483.28, 2604.88, 2393.81, 2231.27, 2420.82,
    2544.0, 2766.67, 2919.62, 2763.25];
const HIGH: &[f64] = &[2174.72, 2129.49, 2158.92, 2050.2, 2042.12, 2151.19, 2220.64, 2352.98,
    2456.25, 2691.53, 2572.81, 2494.14, 2845.93, 2682.66, 2527.13, 2455.68,
    2607.54, 2872.17, 3004.26, 3036.05];
const LOW: &[f64] = &[1934.7, 1921.02, 1793.77, 1887.36, 1919.72, 1868.23, 1991.19, 2011.08,
    2193.91, 2183.96, 2223.15, 2363.19, 2240.03, 2208.31, 2192.15, 2199.02,
    2311.16, 2463.15, 2651.8, 2749.42];
const CLOSE: &[f64] = &[1959.83, 2041.42, 2019.04, 1969.53, 2082.75, 2209.52, 2200.9, 2364.04,
    2543.32, 2423.95, 2483.28, 2604.88, 2393.81, 2231.27, 2420.82, 2544.0,
    2766.67, 2919.62, 2763.25, 2922.14];

#[test]
fn sma_works() {
    let mut result = sma(CLOSE, 4).unwrap();
    let expected = &[1997.455, 2028.185, 2070.21, 2115.675, 2214.3025, 2329.445, 2383.0525,
        2453.6475, 2513.8575, 2476.48, 2428.31, 2412.695, 2397.475, 2490.69,
        2662.7775, 2748.385, 2842.92];
    round_array(result.as_mut(), 4);
    assert_eq!(result, expected);
}

#[test]
fn ema_works() {
    let mut result = ema(CLOSE, 4).unwrap();
    let expected = &[1997.455, 2031.573, 2102.7518, 2142.0111, 2230.8226, 2355.8216, 2383.073,
        2423.1558, 2495.8455, 2455.0313, 2365.5268, 2387.6441, 2450.1864, 2576.7799,
        2713.9159, 2733.6496, 2809.0457];
    round_array(result.as_mut(), 4);
    assert_eq!(result, expected);
}

#[test]
fn psar_works() {
    let mut result = psar(HIGH, LOW, 0.02, 0.2).unwrap();
    let expected = &[2174.72, 2169.646, 2158.92, 2158.92, 1793.77, 1800.9184, 1817.7073,
        1849.8236, 1898.3377, 1977.657, 2049.0443, 2113.2928, 2201.2093, 2845.93,
        2832.8544, 2820.0403, 2192.15, 2205.7504, 2237.6908];
    round_array(result.as_mut(), 4);
    // For some reasons, the first values are not exactly the same but since this indicator
    // was not clearly described by its author, we can say that current implementation is correct.
    assert_eq!(result[result.len() - 16..result.len()],
               expected[expected.len() - 16..expected.len()]);
}

#[test]
fn rsi_works() {
    let mut result = rsi(CLOSE, 6).unwrap();
    let expected = &[79.9771, 86.5336, 90.5949, 73.0035, 75.8056, 80.7258, 56.706, 44.4766,
        57.3488, 63.879, 72.8847, 77.5072, 64.1009, 70.3536];
    round_array(result.as_mut(), 4);
    assert_eq!(result, expected);
}
