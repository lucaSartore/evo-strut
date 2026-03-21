use std::marker::PhantomData;
use anyhow::Result;

mod evolver;
pub use evolver::{Evolver, EvolverBehaviourTrait, EvolverBehaviour};
use rand::Rng;
use rerun::external::glam::usize;

mod score;
pub use score::Cost;

mod tournament_based_crossover_selection;
pub use tournament_based_crossover_selection::{TournamentBasedCrossoverSelection, TournamentBasedCrossoverSelectionSettings};

mod elitist_next_generation_selector;
pub use elitist_next_generation_selector::{ElitistNextGenSelector, ElitistNextGenSelectorSettings};

mod patience_based_termination_strategy;
pub use patience_based_termination_strategy::{PatienceBasedTerminationStrategySettings, PatienceBasedTerminationStrategy};

mod random;
pub use random::Random;


pub trait Mutator<T,S>{
    fn new(settings: &S, rand: Random) -> Self;
    fn mutate(&self, gene: &mut T);
}

pub trait Crossover<T,S>{
    fn new(settings: &S, rand: Random) -> Self;
    fn crossover(&self, a: &T, b: &T) -> T;
}

pub trait TerminationStrategy<S>{
    fn new(settings: &S) -> Self;
    fn should_terminate(&self, best_score: Cost) -> bool;
}

pub trait Evaluator<T,S> {
    fn new(settings: &S) -> Self;
    fn evaluate(&self, gene: &T) -> Cost;
    fn visualize(&self, gene: &T) -> Result<()>;
}

pub trait CrossoverSelector<S> {
    fn new(settings: &S, rand: Random) -> Self;
    /// select n elements (based on the score) for crossover operation
    fn select_for_crossover(&self, scores: &[Cost], n: usize) -> Option<Vec<(usize, usize)>>;
}


pub trait NextGenerationSelector<T,S> {
    fn new(settings: &S, rand: Random) -> Self;
    /// determines how many offspring should be generated from the current generation
    fn num_offspring_to_generate(&self) -> usize;
    fn next_generation(
        &self,
        current_gen: Vec<T>,
        current_gen_costs: Vec<Cost>,
        next_gen: Vec<T>,
        next_gen_costs: Vec<Cost>
    ) -> (Vec<T>, Vec<Cost>);
}

pub trait PopulationInitializer<T,S> {
    fn new(settings: &S, rand: Random) -> Self;
    fn get_initial_individuals(&self) -> usize;
    fn get_random_individual(&self) -> T;
}
