use std::{convert::Infallible, ops::DerefMut, sync::Mutex};
use rand::{TryRng, prelude::*};
use rand_distr::{Distribution, Normal, SkewNormal, Uniform};

use crate::support::random_distribution::RandomDistribution;

pub enum Random {
    // is slower due to mutex, but allow for more reproducibility
    // (better for testing)
    SeededRandom(Box<Mutex<StdRng>>),
    // is faster, but not reproducible
    // (better for actual runs)
    UnSeededRandom
}

impl Random {
    pub fn new(seed: Option<u64>) -> Self {
        match seed {
            Some(s) => Random::SeededRandom(Mutex::new(StdRng::seed_from_u64(s)).into()),
            None => Random::UnSeededRandom
        }
    }
}

// marker trait to avoid verbosity
pub trait R: TryRng<Error = Infallible> {}
impl<T: TryRng<Error = Infallible>> R for T {}


/// simple macro rule to have different implementation
/// regardless of the kind of random we are using (seeded or not)
macro_rules! publish {
    // implementation for non generic functions
    ($public_name: ident = $private_name: ident($($arg:ident: $t: ty), *) -> $return:ty) => {
        pub fn $public_name(&self $(,$arg: $t)*) -> $return{
            if let Random::SeededRandom(r) = self {
                let mut guard = r.lock().expect("Another tread has panicked");
                let r = guard.deref_mut();
                return self.$private_name($($arg, )* r);
            } else {
                let mut r = rand::rng();
                return self.$private_name($($arg, )* &mut r);
            }
        }
    };
    // specialized implementation for generic types (lt is lifetime, gt is generic type)
    ($public_name: ident = $private_name: ident<$($lt:tt), *$($gt:ident), *>($($arg:ident: $t: ty), *) -> $return:ty) => {
        pub fn $public_name<$($lt, )*$($gt, )*>(&self $(,$arg: $t)*) -> $return{
            if let Random::SeededRandom(r) = self {
                let mut guard = r.lock().expect("Another tread has panicked");
                let r = guard.deref_mut();
                return self.$private_name($($arg, )* r);
            } else {
                let mut r = rand::rng();
                return self.$private_name($($arg, )* &mut r);
            }
        }
    }
}

#[allow(dead_code)]
impl Random {

    /// create a copy of the current random
    /// while maintaining reproducibility using seed
    pub fn seeded_copy(&self) -> Self {
        match self {
            Random::UnSeededRandom => Self::new(None),
            Random::SeededRandom(_) => Self::new(Some(self.next_u64()))
        }
    }

    /// Returns a random element from the slice.
    /// Panics if the slice is empty.
    pub fn choose_or_panic<'a, T>(&self, options: &'a [T]) -> &'a T {
        self.choose(options).expect("Cannot choose from an empty vector")
    }


    /// Returns a random element from the slice.
    /// Panics if the slice is empty.
    pub fn next_in_range(&self, low: u64, high: u64) -> u64 {
        self.next_u64() % (high - low) + low
    }


    // Returns a random element from the slice.
    // Panics if the slice is empty.
    publish!(choose = _choose<'a, T>(options: &'a [T]) -> Option<&'a T>);
    fn _choose<'a, T>(&self, options: &'a [T], r: &mut impl R) -> Option<&'a T> {
        options.choose(r)
    }

    // Returns n random elements from the slice.
    // Note: This uses 'choose_multiple', which samples without replacement.
    publish!(choose_many = _choose_many<'a, T>(n: usize, options: &'a [T]) -> Vec<&'a T>);
    pub fn _choose_many<'a, T>(&self, n: usize, options: &'a [T], r: &mut impl R) -> Vec<&'a T> {
        options
            .sample(r, n)
            .collect()
    }


    // Returns n random elements from the slice.
    // Note: This uses 'choose_multiple', which samples without replacement.
    publish!(next_u64 = _next_u64() -> u64);
    fn _next_u64(&self, r: &mut impl R) -> u64 {
        r.next_u64()
    }

    // Returns n random elements from the slice.
    // Note: This uses 'choose_multiple', which samples without replacement.
    publish!(next_u32 = _next_u32() -> u32);
    fn _next_u32(&self, r: &mut impl R) -> u32 {
        r.next_u32()
    }


    // Returns a float in the specified range
    publish!(next_f32 = _next_f32(from: f32, to: f32) -> f32);
    fn _next_f32(&self, from: f32, to: f32, r: &mut impl R) -> f32 {
        Uniform::new(from, to)
            .expect("uniform distribution creation failed")
            .sample(r)
    }

    // generate a random number starting from a distribution
    publish!(next_distribution = _next_distribution(d: &RandomDistribution) -> f32);
    fn _next_distribution(&self, d: &RandomDistribution, r: &mut impl R) -> f32 {
        self._next_distribution_many(d, 1, r)[0]
    }

    // generate many random numbers starting from a distribution
    publish!(next_distribution_many = _next_distribution_many(d: &RandomDistribution, n: usize) -> Vec<f32>);
    fn _next_distribution_many(&self, d: &RandomDistribution, n: usize, r: &mut impl R) -> Vec<f32> {
        match d {
            RandomDistribution::InRange { low, high } => {
                let dist = Uniform::new(low, high).expect("Invalid InRange distribution parameters");
                dist.sample_iter(r).take(n).collect()
            }
            RandomDistribution::Normal { mean, std_dev } => {
                let dist = Normal::new(*mean, *std_dev)
                    .expect("Invalid Normal distribution parameters");
                dist.sample_iter(r).take(n).collect()
            }
            RandomDistribution::SkewNormal { mean, std_dev, shape } => {
                let dist = SkewNormal::new(*mean, *std_dev, *shape)
                    .expect("Invalid SkewNormal distribution parameters");
                dist.sample_iter(r).take(n).collect()
            }
        }
    }
}
