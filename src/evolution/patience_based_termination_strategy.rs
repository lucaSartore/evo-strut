use std::{ops::DerefMut, sync::Mutex};
use super::*;


#[derive(Copy, Clone, Debug)]
pub struct  PatienceBasedTerminationStrategySettings {
    pub max_generations: usize,
    pub patience: usize
}

impl Default for PatienceBasedTerminationStrategySettings {
    fn default() -> Self {
        Self { max_generations: 300, patience: 30 }
    }
}

struct State {
    pub best_so_far: Cost,
    pub best_so_far_generation: usize,
    pub current_generation: usize
}

pub struct PatienceBasedTerminationStrategy {
    settings: PatienceBasedTerminationStrategySettings,
    state: Mutex<State>
}

impl TerminationStrategy<PatienceBasedTerminationStrategySettings> for PatienceBasedTerminationStrategy{
    fn new(settings: &PatienceBasedTerminationStrategySettings) -> Self {
        Self {
            settings: *settings,
            state: State {
                best_so_far: Cost::MAX,
                best_so_far_generation: 0,
                current_generation: 0
            }.into()
        }
    }

    fn should_terminate(&self, best_score: Cost) -> bool {
        let mut lock = self.state
            .lock()
            .expect("another thread has panic");
        let state = lock.deref_mut();

        state.current_generation += 1;
        if best_score < state.best_so_far {
            state.best_so_far = best_score;
            state.best_so_far_generation = state.current_generation;
        }

        let stagnation_period = state.current_generation - state.best_so_far_generation;
        let exceeded_max_gen = state.current_generation > self.settings.max_generations;
        let exhausted_patience = stagnation_period > self.settings.patience;

        exceeded_max_gen || exhausted_patience
    }
}
