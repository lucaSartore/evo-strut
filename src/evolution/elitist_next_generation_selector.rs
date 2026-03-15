use itertools::Itertools;
use log::{self, warn};
use super::*;

#[derive(Debug, Clone, Copy)]
pub struct ElitistNextGenSelectorSettings {
    num_elite_individual: usize,
    num_novel_individual: usize
}

impl Default for ElitistNextGenSelectorSettings {
    fn default() -> Self {
        Self { num_elite_individual: 10, num_novel_individual: 90 }
    }
}

pub struct ElitistNextGenSelector {
    settings: ElitistNextGenSelectorSettings
}

impl ElitistNextGenSelector {
    fn n_best<T>(individuals: Vec<T>, costs: Vec<Cost>, mut n: usize) -> Vec<(T, Cost)> {
        assert_eq!(individuals.len(), costs.len(), "costs and individuals must have the same length");
        let mut sorted: Vec<_> = individuals
            .into_iter()
            .zip(costs)
            .sorted_by_key(|x| x.1)
            .collect();

        if n > sorted.len() {
            warn!(
                "ElitistNextGenSelector: trying to select {} individuals from an array of size {}",
                n,
                sorted.len()
            );
            n = sorted.len();
        }

        _ = sorted.split_off(n);
        sorted
    }
}

impl<T> NextGenerationSelector<T, ElitistNextGenSelectorSettings> for ElitistNextGenSelector  {
    fn new(settings: &ElitistNextGenSelectorSettings, rand: Random) -> Self {
        Self{
            settings: *settings
        }
    }

    fn num_offspring_to_generate(&self) -> usize {
        self.settings.num_elite_individual + self.settings.num_elite_individual
    }

    fn next_generation(
        &self,
        current_gen: Vec<T>,
        current_gen_costs: Vec<Cost>,
        next_gen: Vec<T>,
        next_gen_costs: Vec<Cost>
    ) -> (Vec<T>, Vec<Cost>) {
        let elite = Self::n_best(current_gen, current_gen_costs, self.settings.num_elite_individual);
        let novel = Self::n_best(next_gen, next_gen_costs, self.settings.num_novel_individual);
        elite
            .into_iter()
            .chain(novel)
            .unzip()
    }
}
