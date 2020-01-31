use std::f64::INFINITY;
use std::fmt;

use crate::errors::*;
use crate::{Low, Next, Reset};

/// Returns the lowest value in a given time frame.
///
/// # Parameters
///
/// * _n_ - size of the time frame (integer greater than 0). Default value is 14.
///
/// # Example
///
/// ```
/// use quantaxis_rs::indicators::LLV;
/// use quantaxis_rs::Next;
///
/// let mut min = llv::new(3).unwrap();
/// assert_eq!(min.next(10.0), 10.0);
/// assert_eq!(min.next(11.0), 10.0);
/// assert_eq!(min.next(12.0), 10.0);
/// assert_eq!(min.next(13.0), 11.0);
/// ```
#[derive(Debug, Clone)]
pub struct LLV {
    n: usize,
    vec: Vec<f64>,
    min_index: usize,
    cur_index: usize,
    cached: Vec<f64>
}

impl LLV {
    pub fn new(n: u32) -> Result<Self> {
        let n = n as usize;

        if n <= 0 {
            return Err(Error::from_kind(ErrorKind::InvalidParameter));
        }

        let indicator = Self {
            n: n,
            vec: vec![INFINITY; n],
            min_index: 0,
            cur_index: 0,
            cached: vec![],
        };

        Ok(indicator)
    }
    pub fn new_init(n: u32, vec: Vec<f64>) -> Result<Self> {
        let n = n as usize;

        if n <= 0 {
            return Err(Error::from_kind(ErrorKind::InvalidParameter));
        }
//        let mut u =  vec![INFINITY; n];
        let len = vec.len();
//        if len >= n {
//            println!("pre Enough");
//
//            for data in vec[len- n..len].iter(){
//                u.push(*data);
//            }
//        } else {
//            println!("pre not Enough");
//            for data in vec {
//                u.push(data);
//            }
//        }

        let mut indicator = Self {
            n: n,
            vec: vec![INFINITY; n],
            min_index: 0,
            cur_index: 0,
            cached: vec![],
        };
        for data in vec{
            indicator.next(data as f64);
        }
        Ok(indicator)
    }

    fn find_min_index(&self) -> usize {
        let mut min = ::std::f64::INFINITY;
        let mut index: usize = 0;

        for (i, &val) in self.vec.iter().enumerate() {
            if val < min {
                min = val;
                index = i;
            }
        }

        index
    }
}

impl Next<f64> for LLV {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.cur_index = (self.cur_index + 1) % (self.n as usize);
        self.vec[self.cur_index] = input;

        if input < self.vec[self.min_index] {
            self.min_index = self.cur_index;
        } else if self.min_index == self.cur_index {
            self.min_index = self.find_min_index();
        }
        self.cached.push(self.vec[self.min_index]);
        self.vec[self.min_index]
    }
}

impl<'a, T: Low> Next<&'a T> for LLV {
    type Output = f64;

    fn next(&mut self, input: &'a T) -> Self::Output {
        self.next(input.low())
    }
}

impl Reset for LLV {
    fn reset(&mut self) {
        for i in 0..self.n {
            self.vec[i] = INFINITY;
        }
    }
}

impl Default for LLV {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl fmt::Display for LLV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MIN({})", self.n)
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
    test_indicator!(LLV);

    #[test]
    fn test_new() {
        assert!(LLV::new(0).is_err());
        assert!(LLV::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut min = LLV::new(3).unwrap();

        assert_eq!(min.next(4.0), 4.0);
        assert_eq!(min.next(1.2), 1.2);
        assert_eq!(min.next(5.0), 1.2);
        assert_eq!(min.next(3.0), 1.2);
        assert_eq!(min.next(4.0), 3.0);
        assert_eq!(min.next(6.0), 3.0);
        assert_eq!(min.next(7.0), 4.0);
        assert_eq!(min.next(8.0), 6.0);
        assert_eq!(min.next(-9.0), -9.0);
        assert_eq!(min.next(0.0), -9.0);
    }

    #[test]
    fn test_next_with_bars() {
        fn bar(low: f64) -> Bar {
            Bar::new().low(low)
        }

        let mut LLV_Indicator = LLV::new(3).unwrap();
        println!("{:#?}", LLV_Indicator);
        assert_eq!(LLV_Indicator.next(&bar(4.0)), 4.0);
        assert_eq!(LLV_Indicator.next(&bar(4.0)), 4.0);
        println!("{:#?}", LLV_Indicator);
        assert_eq!(LLV_Indicator.next(&bar(1.2)), 1.2);
        assert_eq!(LLV_Indicator.next(&bar(5.0)), 1.2);
        println!("{:#?}", LLV_Indicator);
    }

    #[test]
    fn test_reset() {
        let mut min = LLV::new(10).unwrap();

        assert_eq!(min.next(5.0), 5.0);
        assert_eq!(min.next(7.0), 5.0);

        min.reset();
        assert_eq!(min.next(8.0), 8.0);
    }

    #[test]
    fn test_newx() {
        let mut LLV_Indicator = LLV::new_init(2, vec![3.0,4.0,3.0]).unwrap();
        println!("{:#?}", LLV_Indicator);
        assert_eq!(LLV_Indicator.next(5.0), 3.0);
        println!("{:#?}", LLV_Indicator);
        assert_eq!(LLV_Indicator.next(7.0), 5.0);
        println!("{:#?}", LLV_Indicator);
        LLV_Indicator.reset();
        assert_eq!(LLV_Indicator.next(8.0), 8.0);
    }

    #[test]
    fn test_new_notenough() {
        let mut LLV_Indicator = LLV::new_init(4, vec![4.0,6.0,3.0]).unwrap();
        println!("{:#?}", LLV_Indicator);
        assert_eq!(LLV_Indicator.next(5.0), 3.0);
        println!("{:#?}", LLV_Indicator);
        assert_eq!(LLV_Indicator.next(2.0), 2.0);
        println!("{:#?}", LLV_Indicator);
        LLV_Indicator.reset();
        assert_eq!(LLV_Indicator.next(8.0), 8.0);
    }
    #[test]
    fn test_default() {
        LLV::default();
    }

    #[test]
    fn test_display() {
        let indicator = LLV::new(10).unwrap();
        println!("{}", indicator);
        assert_eq!(format!("{}", indicator), "MIN(10)");
    }
}
