use crate::{evolution::{Crossover, Random}, models::Settings};
use super::CompressedSupportGene;


pub struct SupportStructureCrossoverSettings<'a> {
    settings: &'a Settings
}

impl<'a> SupportStructureCrossoverSettings<'a> {
    pub fn new(settings: &'a Settings) -> Self {
        Self {
            settings
        }
    }
}

pub struct SupportStructureCrossover<'a> {
    settings: &'a Settings,
    rand: Random
}

impl<'a> Crossover<CompressedSupportGene, SupportStructureCrossoverSettings<'a>> for SupportStructureCrossover<'a> {
    fn new(settings: &SupportStructureCrossoverSettings<'a>, rand: Random) -> Self {
        Self {
            settings: settings.settings,
            rand
        }
    }

    fn crossover(&self, a: &CompressedSupportGene, _b: &CompressedSupportGene) -> CompressedSupportGene {
        // todo: implement real crossover
        a.clone()
    }
}
