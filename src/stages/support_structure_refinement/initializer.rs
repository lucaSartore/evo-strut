use crate::{evolution::PopulationInitializer, models::Settings, stages::support_structure_refinement::SupportStructureGene};


pub struct SupportStructureInitializerSettings<'a> {
    settings: &'a Settings,
    template: &'a SupportStructureGene
}

impl<'a> SupportStructureInitializerSettings<'a> {
    pub fn new(settings: &'a Settings, template: &'a SupportStructureGene) -> Self {
        Self {
            settings,
            template
        }
    }
}

pub struct SupportStructureInitializer<'a> {
    settings: &'a Settings,
    template: &'a SupportStructureGene
}

impl<'a> PopulationInitializer<SupportStructureGene, SupportStructureInitializerSettings<'a>> for SupportStructureInitializer<'a> {
    fn new(settings: &SupportStructureInitializerSettings<'a>, _rand: crate::evolution::Random) -> Self {
        Self {
            settings: settings.settings,
            template: settings.template
        }
    }

    fn get_initial_individuals(&self) -> usize {
        self.settings.support_structure_optimization_settings.generation_size
    }

    fn get_random_individual(&self) -> SupportStructureGene {
        self.template.clone()
    }
}
