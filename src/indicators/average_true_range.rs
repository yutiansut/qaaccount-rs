use std::fmt;

use crate::errors::*;
use crate::indicators::{MovingAverage, TrueRange};
use crate::{Close, High, Low, Next, Reset, Update};
use std::f64::INFINITY;

/// Average true range (ATR).
///
/// A technical analysis volatility indicator, originally developed by J. Welles Wilder.
/// The average true range is an N-day smoothed moving average of the true range values.
/// This implementation uses exponential moving average.
///
/// # Formula
///
/// ATR(length)<sub>t</sub> = EMA(length) of TR<sub>t</sub>
///
/// Where:
///
/// * _EMA(n)_ - [exponential moving average](struct.ExponentialMovingAverage.html) with smoothing period _length_
/// * _TR<sub>t</sub>_ - [true range](struct.TrueRange.html) for period _t_
///
/// # Parameters
///
/// * _length_ - smoothing period of EMA (integer greater than 0)
///
/// }
#[derive(Debug, Clone)]
pub struct AverageTrueRange {
    true_range: TrueRange,
    ma: MovingAverage,
    length: usize,
    pub cached: Vec<f64>
}

impl AverageTrueRange {
    pub fn new(length: u32) -> Result<Self> {
        let indicator = Self {
            true_range: TrueRange::new(),
            ma: MovingAverage::new(length)?,
            length: length as usize,
            cached: vec![-INFINITY; length as usize]
        };
        Ok(indicator)
    }
}

impl Next<f64> for AverageTrueRange {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let res = self.ma.next(self.true_range.next(input));
        self.cached.push(res);
        self.cached.remove(0);
        res
    }
}
impl Update<f64> for AverageTrueRange {
    type Output = f64;

    fn update(&mut self, input: f64) -> Self::Output {
        let res = self.ma.update(self.true_range.update(input));
        let x = self.cached.last_mut().unwrap();
        *x = res;
        res
    }
}

impl<'a, T: High + Low + Close> Next<&'a T> for AverageTrueRange {
    type Output = f64;

    fn next(&mut self, input: &'a T) -> Self::Output {
        let res = self.ma.next(self.true_range.next(input));
        self.cached.push(res);
        self.cached.remove(0);
        res
    }
}

impl<'a, T: High + Low + Close> Update<&'a T> for AverageTrueRange {
    type Output = f64;

    fn update(&mut self, input: &'a T) -> Self::Output {
        let res  = self.ma.update(self.true_range.update(input));
        let x = self.cached.last_mut().unwrap();
        *x = res;
        res
    }
}

impl Reset for AverageTrueRange {
    fn reset(&mut self) {
        self.true_range.reset();
        self.ma.reset();
    }
}

impl Default for AverageTrueRange {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl fmt::Display for AverageTrueRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ATR({})", self.ma.n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;
    macro_rules! test_indicator {
        ($i:tt) => {
            #[test]
            fn test_indicator() {
                let bar = Bar::new();

                // ensure Default trait is implemented
                let mut indicator = $i::default();

                // ensure Next<f64> is implemented
                let first_output = indicator.next(12.3);

                // ensure next accepts &DataItem as well
                indicator.next(&bar);

                // ensure Reset is implemented and works correctly
                indicator.reset();
                assert_eq!(indicator.next(12.3), first_output);

                // ensure Display is implemented
                format!("{}", indicator);
            }
        };
    }
    test_indicator!(AverageTrueRange);

    #[test]
    fn test_new() {
        assert!(AverageTrueRange::new(0).is_err());
        assert!(AverageTrueRange::new(1).is_ok());
    }
    #[test]
    fn test_next() {
        let mut atr = AverageTrueRange::new(3).unwrap();

        let bar1 = Bar::new().high(10).low(7.5).close(9);
        let bar2 = Bar::new().high(11).low(9).close(9.5);
        let bar3 = Bar::new().high(9).low(5).close(8);

        assert_eq!(atr.next(&bar1), 0f64);
        assert_eq!(atr.next(&bar2), 0f64);
        assert_eq!(atr.next(&bar3), 3f64);
    }

    #[test]
    fn test_reset() {
        let mut atr = AverageTrueRange::new(9).unwrap();

        let bar1 = Bar::new().high(10).low(7.5).close(9);
        let bar2 = Bar::new().high(11).low(9).close(9.5);

        atr.next(&bar1);
        atr.next(&bar2);

        atr.reset();
        let bar3 = Bar::new().high(60).low(15).close(51);
        assert_eq!(atr.next(&bar3), 0.0);
    }

    #[test]
    fn test_default() {
        AverageTrueRange::default();
    }

    #[test]
    fn test_display() {
        let indicator = AverageTrueRange::new(8).unwrap();
        assert_eq!(format!("{}", indicator), "ATR(8)");
    }
}
