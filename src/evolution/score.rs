use log::{self, warn};

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct  Cost {
    score: f32
}

impl Cost {
   pub fn new(score: f32) -> Cost {
       if f32::is_nan(score) {
           warn!("trying to build a cost with NaN: {score}. Defaulting to f32::MAX");
           return Self::MAX;
       }
       Cost { score }
   }

   pub const MAX: Cost = Cost {score: f32::MAX};
}

impl Eq for Cost { }
impl Ord for Cost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("Cost should never be built with NaN")
    }
}
