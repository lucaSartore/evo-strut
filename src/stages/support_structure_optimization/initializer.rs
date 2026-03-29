use crate::{evolution::PopulationInitializer, models::Settings, stages::support_structure_optimization::SupportStructureGene};


pub struct SupportStructureInitializerSettings<'a> {
    settings: &'a Settings
}

impl<'a> SupportStructureInitializerSettings<'a> {
    pub fn new(settings: &'a Settings) -> Self {
        Self {
            settings
        }
    }
}

pub struct SupportStructureInitializer<'a> {
    settings: &'a Settings
}

impl<'a> PopulationInitializer<SupportStructureGene, SupportStructureInitializerSettings<'a>> for SupportStructureInitializer<'a> {
    fn new(settings: &SupportStructureInitializerSettings<'a>, rand: crate::evolution::Random) -> Self {
        todo!()
    }

    fn get_initial_individuals(&self) -> usize {
        todo!()
    }

    fn get_random_individual(&self) -> SupportStructureGene {
        todo!()
    }
}
