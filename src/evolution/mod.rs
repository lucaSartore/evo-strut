use std::marker::PhantomData;

mod evolver;
pub use evolver::Evolver;
use crate::models::Settings;
use rand::Rng;
use rerun::external::glam::usize;



pub trait Mutator<T,S>{
    fn new(settings: S, rand: &(impl Rng + Sync)) -> Self;
    fn mutate(&self, gene: T) -> T;
}

pub trait Crossover<T,S>{
    fn new(settings: S, rand: &(impl Rng + Sync)) -> Self;
    fn crossover(&self, a: &T, b: &T) -> T;
}

pub trait TerminationStrategy<S>{
    fn new(settings: S) -> Self;
    fn should_terminate(&self, best_score: f32) -> bool;
}

pub trait Evaluator<T,S> {
    fn new(settings: S) -> Self;
    fn evaluate(&self, gene: &T) -> f32;
}

pub trait CrossoverSelector<S> {
    fn new(settings: S, rand: &(impl Rng + Sync)) -> Self;
    /// select n elements (based on the score) for crossover operation
    fn select_for_crossover(&self, scores: &Vec<f32>, n: usize) -> Vec<(usize, usize)>;
}


pub trait NextGenerationSelector<T,S> {
    fn new(settings: S, rand: &(impl Rng + Sync)) -> Self;
    /// determines how many offspring should be generated from the current generation
    fn num_offspring_to_generate(&self) -> usize;
    fn next_generation(
        &self,
        current_gen: Vec<T>,
        current_gen_scores: Vec<f32>,
        next_gen: Vec<T>,
        next_gen_scores: Vec<f32>
    ) -> (Vec<T>, Vec<f32>);
}

pub trait PopulationInitializer<T,S> {
    fn new(settings: S, rand: &(impl Rng + Sync)) -> Self;
    fn get_initial_individuals(&self) -> usize;
    fn get_random_individual(&self) -> T;
}
