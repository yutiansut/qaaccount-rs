
use core::fmt::Debug;
use core::ops::AddAssign;

extern crate num_traits;
use num_traits::{float::Float, identities::Zero, identities::One, cast::FromPrimitive};

#[macro_use]
extern crate serde;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Data<T: Float + Zero + One + AddAssign + FromPrimitive + PartialEq + Debug> {
    pub min:     T,
    pub max:     T,
    pub mean:    T,
    
    /// 
    pub std_dev: T,

    /// count 序列号
    #[serde(skip)]
    count: usize,

    /// Internal mean squared for algo
    #[serde(skip)]
    mean2:   T,
}

impl <T> Data<T>
    where
        T: Float + Zero + One + AddAssign + FromPrimitive + PartialEq + Debug,
{
    pub fn new() -> Data<T> {
        Data{count: 0, min: T::zero(), max: T::zero(), mean: T::zero(), std_dev: T::zero(), mean2: T::zero()}
    }

    pub fn update(&mut self, value: T) {
        // Track min and max
        if value > self.max || self.count == 0 {
            self.max = value;
        }
        if value < self.min || self.count == 0 {
            self.min = value;
        }

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



fn main(){
    let mut s: Data<f32> = Data::new();

    let vals: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    for v in &vals {
        s.update(*v);
        println!("update new vals {}", v);
        println!("max {:?}", s.max);
        println!("mean {:?}", s.mean);
    }

    
}