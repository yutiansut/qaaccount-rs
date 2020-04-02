use core::fmt::Debug;
use core::ops::AddAssign;

use ndarray::array;
use num_traits::{cast::FromPrimitive, float::Float, identities::One, identities::Zero};
use serde::{Deserialize, Serialize};

//use num_traits::real::Real;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stats<T: Float + Zero + One + AddAssign + FromPrimitive + PartialEq + Debug> {
    pub min: T,
    pub max: T,
    /// Mean of sample set
    pub mean: T,
    /// Standard deviation of sample
    pub std_dev: T,

    /// Number of values collected
    #[serde(skip)]
    count: usize,

    /// Internal mean squared for algo
    #[serde(skip)]
    mean2: T,
}

impl<T> Stats<T>
    where
        T: Float + Zero + One + AddAssign + FromPrimitive + PartialEq + Debug,
{
    /// Create a new rolling-stats object
    pub fn new() -> Stats<T> {
        Stats {
            count: 0,
            min: T::zero(),
            max: T::zero(),
            mean: T::zero(),
            std_dev: T::zero(),
            mean2: T::zero(),
        }
    }

    /// Update the rolling-stats object
    pub fn update(&mut self, value: T) {
        // Track min and max
        if value > self.max || self.count == 0 {
            self.max = value;
        }
        if value < self.min || self.count == 0 {
            self.min = value;
        }

        // Increment counter
        self.count += 1;
        let count = T::from_usize(self.count).unwrap();

        // Calculate mean
        let delta: T = value - self.mean;
        self.mean += delta / count;

        // Mean2 used internally for standard deviation calculation
        let delta2: T = value - self.mean;
        self.mean2 += delta * delta2;

        // Calculate standard deviation
        if self.count > 1 {
            self.std_dev = (self.mean2 / (count - T::one())).sqrt();
        }
    }
}

fn main() {
    let mut s: Stats<f32> = Stats::new();

    let vals: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    for v in &vals {
        s.update(*v);

        println!("{:?}", s.max);
        println!("{:?}", s.mean);
    }
}
