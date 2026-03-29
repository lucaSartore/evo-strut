use crate::{evolution::Crossover, models::Settings, stages::support_structure_optimization::SupportStructureGene};


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
    settings: &'a Settings
}

impl<'a> Crossover<SupportStructureGene, SupportStructureCrossoverSettings<'a>> for SupportStructureCrossover<'a> {
    fn new(settings: &SupportStructureCrossoverSettings<'a>, rand: crate::evolution::Random) -> Self {
        todo!()
    }

    fn crossover(&self, a: &SupportStructureGene, b: &SupportStructureGene) -> SupportStructureGene {
        todo!()
    }
}
