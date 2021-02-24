use std::fmt;

use crate::helpers::max3;
use crate::{Close, High, Low, Next, Reset, Update};


#[derive(Debug, Clone)]
pub struct TrueRange {


    prev_closeque: Vec<f64>

}

impl TrueRange {
    pub fn new() -> Self {
        Self {  prev_closeque:vec![] }
    }
}

impl Default for TrueRange {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TrueRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TRUE_RANGE()")
    }
}

impl Next<f64> for TrueRange {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        if self.prev_closeque.len()<1{
            self.prev_closeque.push(input);
            0.0
        }else {
            let distance = match self.prev_closeque[self.prev_closeque.len() - 1] {
                prev => (input - prev).abs(),
                _ => 0.0
            };

            self.prev_closeque.push(input);
            distance
        }


    }
}
impl Update<f64> for TrueRange {
    type Output = f64;

    fn update(&mut self, input: f64) -> Self::Output {

        if self.prev_closeque.len()<2{
            let u = self.prev_closeque.last_mut().unwrap();
            *u = input;
            0.0
        }else{
            let distance = match self.prev_closeque[self.prev_closeque.len() -2]{
                prev => (input - prev).abs(),
                _ => 0.0
            };

            let u = self.prev_closeque.last_mut().unwrap();
            *u = input;
            //self.prev_close = Some(input);
            distance
        }

    }
}

impl<'a, T: High + Low + Close> Next<&'a T> for TrueRange {
    type Output = f64;

    fn next(&mut self, bar: &'a T) -> Self::Output {
        if self.prev_closeque.len()<1{
            self.prev_closeque.push(bar.close());
            bar.high()- bar.low()
        }else{
            let max_dist = match self.prev_closeque[self.prev_closeque.len() - 1] {
                prev_close => {
                    let dist1 = bar.high() - bar.low();
                    let dist2 = (bar.high() - prev_close).abs();
                    let dist3 = (bar.low() - prev_close).abs();
                    max3(dist1, dist2, dist3)
                }
                _ => bar.high() - bar.low(),
            };
            self.prev_closeque.push(bar.close());
            max_dist
        }
    }
}
impl<'a, T: High + Low + Close> Update<&'a T> for TrueRange {
    type Output = f64;

    fn update(&mut self, bar: &'a T) -> Self::Output {
        if self.prev_closeque.len()<2{
            let u = self.prev_closeque.last_mut().unwrap();
            *u = bar.close();
            bar.high() - bar.low()
        }else{
            let max_dist = match self.prev_closeque[self.prev_closeque.len() -2] {
                prev_close => {
                    let dist1 = bar.high() - bar.low();
                    let dist2 = (bar.high() - prev_close).abs();
                    let dist3 = (bar.low() - prev_close).abs();
                    max3(dist1, dist2, dist3)
                }
                _ => bar.high() - bar.low(),
            };
            let u = self.prev_closeque.last_mut().unwrap();
            *u = bar.close();
            max_dist
        }

    }
}

impl Reset for TrueRange {
    fn reset(&mut self) {
        self.prev_closeque = vec![];
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
    test_indicator!(TrueRange);

    #[test]
    fn test_next_f64() {
        let mut tr = TrueRange::new();
        assert_eq!(round(tr.next(2.5)), 0.0);
        assert_eq!(round(tr.next(3.6)), 1.1);
        assert_eq!(round(tr.next(3.3)), 0.3);
    }



    #[test]
    fn test_update_f64() {
        let mut tr = TrueRange::new();
        assert_eq!(round(tr.next(2.5)), 0.0);

        assert_eq!(round(tr.update(3.3)), 0.0);


        let mut tr = TrueRange::new();
        assert_eq!(round(tr.next(2.5)), 0.0);
        println!("{:#?}", tr);
        assert_eq!(round(tr.update(3.3)), 0.0);
        println!("{:#?}", tr);
        // println!("{:#?}",tr.next(2.5));
        assert_eq!(round(tr.next(2.5)), 0.8);
        println!("{:#?}", tr);
        assert_eq!(round(tr.update(3.6)), 0.3);
        println!("{:#?}", tr);

    }

    #[test]
    fn test_next_bar() {
        let mut tr = TrueRange::new();

        let bar1 = Bar::new().high(10).low(7.5).close(9);
        let bar2 = Bar::new().high(11).low(9).close(9.5);
        let bar3 = Bar::new().high(9).low(5).close(8);

        assert_eq!(tr.next(&bar1), 2.5);
        assert_eq!(tr.next(&bar2), 2.0);
        assert_eq!(tr.next(&bar3), 4.5);
    }

    #[test]
    fn test_reset() {
        let mut tr = TrueRange::new();

        let bar1 = Bar::new().high(10).low(7.5).close(9);
        let bar2 = Bar::new().high(11).low(9).close(9.5);

        tr.next(&bar1);
        tr.next(&bar2);

        tr.reset();
        let bar3 = Bar::new().high(60).low(15).close(51);
        assert_eq!(tr.next(&bar3), 45.0);
    }

    #[test]
    fn test_default() {
        TrueRange::default();
    }

    #[test]
    fn test_display() {
        let indicator = TrueRange::new();
        assert_eq!(format!("{}", indicator), "TRUE_RANGE()");
    }
}
