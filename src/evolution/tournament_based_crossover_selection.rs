use std::cell::RefCell;

use crate::evolution::Random;
use super::*;

#[derive(Copy, Clone, Debug)]
pub struct TournamentBasedCrossoverSelectionSettings {
    pub k: usize
}
pub struct TournamentBasedCrossoverSelection {
    settings: TournamentBasedCrossoverSelectionSettings,
    rand: Random
}

impl CrossoverSelector<TournamentBasedCrossoverSelectionSettings> for TournamentBasedCrossoverSelection {
    fn new(settings: &TournamentBasedCrossoverSelectionSettings, rand: Random) -> Self {
        TournamentBasedCrossoverSelection {
            settings: settings.clone(),
            rand
        }
    }

    fn select_for_crossover(&self, scores: &[Cost], n: usize) -> Option<Vec<(usize, usize)>> {
        let mut to_return = vec![];
        for _ in 0..n {
            let parent_one_options = self.rand.choose_many(self.settings.k, scores);
            let parent_one = parent_one_options
                .iter()
                .enumerate()
                .min_by_key(|x| x.1)?
                .0;

            let parent_two_options = self.rand.choose_many(self.settings.k, scores);
            let parent_two = parent_two_options
                .iter()
                .enumerate()
                .min_by_key(|x| x.1)?
                .0;

            to_return.push((parent_one, parent_two));
        }
        Some(to_return)
    }
}
