use crate::{evolution::Mutator, models::Settings, stages::support_structure_optimization::SupportStructureGene};


pub struct SupportStructureMutatorSettings<'a> {
    settings: &'a Settings
}

impl<'a> SupportStructureMutatorSettings<'a> {
    pub fn new(settings: &'a Settings) -> Self {
        Self {
            settings
        }
    }
}


pub struct SupportStructureMutator<'a> {
    settings: &'a Settings
}

impl<'a> Mutator<SupportStructureGene, SupportStructureMutatorSettings<'a>> for SupportStructureMutator<'a> {
    fn new(settings: &SupportStructureMutatorSettings<'a>, rand: crate::evolution::Random) -> Self {
        todo!()
    }

    fn mutate(&self, gene: &mut SupportStructureGene) {
        todo!()
    }
}
