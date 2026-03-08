use std::ops::Add;

use log::{self, warn};

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
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
}

impl Eq for Cost { }
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
