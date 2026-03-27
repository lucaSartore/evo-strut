use log::error;

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

impl TournamentBasedCrossoverSelection {
    fn tournament_selection(&self, scores: &[Cost],) -> Option<usize> {
        let options = self.rand.choose_many(self.settings.k, scores);
        let element = options
            .iter()
            .enumerate()
            .min_by_key(|x| x.1);
        if element.is_none() {
            error!("TournamentBasedCrossoverSelection: trying to make tournament with empty size");
        }
        Some(element?.0)
    }
}

impl CrossoverSelector<TournamentBasedCrossoverSelectionSettings> for TournamentBasedCrossoverSelection {
    fn new(settings: &TournamentBasedCrossoverSelectionSettings, rand: Random) -> Self {
        TournamentBasedCrossoverSelection {
            settings: *settings,
            rand
        }
    }

    fn select_for_crossover(&self, scores: &[Cost], n: usize) -> Option<Vec<(usize, usize)>> {
        let mut to_return = vec![];
        for _ in 0..n {
            let a = self.tournament_selection(scores)?;
            let b = self.tournament_selection(scores)?;
            to_return.push((a, b));
        }
        Some(to_return)
    }
}
