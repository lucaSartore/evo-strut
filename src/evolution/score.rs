use std::{fmt::Display, ops::Add};

use log::{self, warn};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct  Cost {
    cost: f32
}

impl Cost {
   pub fn new(cost: f32) -> Cost {
       if f32::is_nan(cost) {
           warn!("trying to build a cost with NaN: {cost}. Defaulting to f32::MAX");
           return Self::MAX;
       }
       Cost { cost }
   }

   pub const MAX: Cost = Cost {cost: f32::MAX};
   pub const ZERO: Cost = Cost {cost: 0.};
   pub fn as_f32(&self) -> f32 {
       self.cost
   }
   pub fn times(mut self, value: f32) -> Cost {
       self.cost *= value;
       if f32::is_nan(self.cost) {
           warn!("trying to multiply a cost with NaN result. Defaulting to f32::MAX");
           return Self::MAX;
       }
       self
   }
}

impl Display for Cost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.cost.fmt(f)
    }
}

impl Eq for Cost { }
impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}
impl Ord for Cost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("Cost should never be built with NaN")
    }
}

impl Add<Cost> for Cost {
    type Output = Cost;

    fn add(self, rhs: Cost) -> Self::Output {
        Cost::new(self.cost + rhs.cost)
    }
}
