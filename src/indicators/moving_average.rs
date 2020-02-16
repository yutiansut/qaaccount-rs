use std::f64::NAN;
use std::fmt;

use crate::{Close, Next, Reset, Update};
use crate::errors::*;

/// moving average (ma).
///
/// # Formula
///
/// ![ma](https://wikimedia.org/api/rest_v1/media/math/render/svg/e2bf09dc6deaf86b3607040585fac6078f9c7c89)
///
/// Where:
///
/// * _MA<sub>t</sub>_ - value of moving average at a point of time _t_
/// * _n_ - number of periods (length)
/// * _p<sub>t</sub>_ - input value at a point of time _t_
///
/// # Parameters
///
/// * _n_ - number of periods (integer greater than 0)
///
/// # Example
///
/// ```
/// use quantaxis_rs::indicators::MovingAverage;
/// use quantaxis_rs::Next;
///
/// let mut ma = MovingAverage::new(3).unwrap();
/// assert_eq!(ma.next(10.0), 0.0);
/// assert_eq!(ma.next(11.0), 0.0);
/// assert_eq!(ma.next(12.0), 11.0);
/// assert_eq!(ma.next(13.0), 12.0);
/// ```
///
#[derive(Debug, Clone)]
pub struct MovingAverage {
    n: u32,
    index: usize,
    count: u32,
    sum: f64,
    vec: Vec<f64>,
    pub cached: Vec<f64>
}

impl MovingAverage {
    pub fn new(n: u32) -> Result<Self> {
        match n {
            0 => Err(Error::from_kind(ErrorKind::InvalidParameter)),
            _ => {
                let indicator = Self {
                    n,
                    index: 0,
                    count: 1,
                    sum: 0.0,
                    vec: vec![0.0; n as usize],
                    cached: vec![0.0; n as usize],
                };
                Ok(indicator)
            }
        }
    }
}

impl Next<f64> for MovingAverage {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.index = (self.index + 1) % (self.n as usize);

        let old_val = self.vec[self.index];
        self.vec[self.index] = input;
        let mut res = 0.0;

        if self.count < self.n {
            self.sum = self.sum - old_val + input;
        } else {
            self.sum = self.sum - old_val + input;
            res = self.sum / (self.n as f64);
        }
        self.count += 1;
        self.cached.push(res);
        self.cached.remove(0);
        res
    }
}


impl<'a, T: Close> Next<&'a T> for MovingAverage {
    type Output = f64;

    fn next(&mut self, input: &'a T) -> Self::Output {
        self.next(input.close())
    }
}

impl Update<f64> for MovingAverage {
    type Output = f64;

    fn update(&mut self, input: f64) -> Self::Output {
        //self.index = (self.index + 1) % (self.n as usize);

        let old_val = self.vec[self.index];
        self.vec[self.index] = input;
        let mut res = 0.0;
        self.count -= 1;
        if self.count < self.n {
            self.sum = self.sum - old_val + input;
        } else {
            self.sum = self.sum - old_val + input;
            res = self.sum / (self.n as f64);
        }
        self.count += 1;
        //println!("VEC {:#?} index {}, count {}", self.vec, self.index, self.count);
        self.cached.remove((self.n - 1) as usize);
        self.cached.push(res);
        res
    }
}


impl<'a, T: Close> Update<&'a T> for MovingAverage {
    type Output = f64;

    fn update(&mut self, input: &'a T) -> Self::Output {
        self.update(input.close())
    }
}


impl Reset for MovingAverage {
    fn reset(&mut self) {
        self.index = 0;
        self.count = 0;
        self.sum = 0.0;
        for i in 0..(self.n as usize) {
            self.vec[i] = 0.0;
            self.cached[i] = 0.0;
        }

    }
}

impl Default for MovingAverage {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

impl fmt::Display for MovingAverage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ma({})", self.n)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::*;

    use super::*;

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
    test_indicator!(MovingAverage);

    #[test]
    fn test_new() {
        assert!(MovingAverage::new(0).is_err());
        assert!(MovingAverage::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut ma = MovingAverage::new(4).unwrap();
        assert_eq!(ma.next(4.0), 0.0);
        assert_eq!(ma.next(5.0), 0.0);
        assert_eq!(ma.next(6.0), 0.0);
        assert_eq!(ma.next(6.0), 5.25);
        assert_eq!(ma.next(6.0), 5.75);
        assert_eq!(ma.next(6.0), 6.0);
        assert_eq!(ma.next(2.0), 5.0);
    }

    #[test]
    fn test_update() {
        let mut ma = MovingAverage::new(2).unwrap();
        assert_eq!(ma.next(5.0), 0.0);
        assert_eq!(ma.update(6.0), 0.0);
        assert_eq!(ma.next(7.0), 6.5);
        assert_eq!(ma.update(8.0), 7.0);
        assert_eq!(ma.next(9.0), 8.5);
    }


    #[test]
    fn test_cached() {
        let mut ma = MovingAverage::new(2).unwrap();
        assert_eq!(ma.next(4.0), 0.0);
        assert_eq!(ma.next(5.0), 4.5);
        assert_eq!(ma.next(6.0), 5.5);
        println!("{:#?}", ma.cached);
        assert_eq!(ma.next(6.0), 6.0);
        println!("{:#?}", ma.cached);
        assert_eq!(ma.next(6.0), 6.0);
        println!("{:#?}", ma.cached);
        assert_eq!(ma.next(6.0), 6.0);
        println!("{:#?}", ma.cached);
        assert_eq!(ma.next(2.0), 4.0);
        println!("{:#?}", ma.cached);
    }

    #[test]
    fn test_next_with_bars() {
        fn bar(close: f64) -> Bar {
            Bar::new().close(close)
        }

        let mut ma = MovingAverage::new(3).unwrap();
        assert_eq!(ma.next(&bar(4.0)), 0.0);
        assert_eq!(ma.next(&bar(4.0)), 0.0);
        assert_eq!(ma.next(&bar(7.0)), 5.0);
        assert_eq!(ma.next(&bar(1.0)), 4.0);
    }

    #[test]
    fn test_reset() {
        let mut ma = MovingAverage::new(4).unwrap();
        assert_eq!(ma.next(4.0), 0.0);
        assert_eq!(ma.next(5.0), 0.0);
        assert_eq!(ma.next(6.0), 0.0);
        assert_eq!(ma.next(5.0), 5.0);
        ma.reset();
        assert_eq!(ma.next(99.0), 0.0);
    }

    #[test]
    fn test_default() {
        MovingAverage::default();
    }

    #[test]
    fn test_display() {
        let ma = MovingAverage::new(5).unwrap();
        assert_eq!(format!("{}", ma), "ma(5)");
    }
}
