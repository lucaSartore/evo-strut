use crate::{evolution::PopulationInitializer, models::Settings, stages::{contact_point_optimization::ContactPointsGene, support_structure_optimization::CompressedSupportGene}};


pub struct SupportStructureInitializerSettings<'a> {
    settings: &'a Settings,
    contact_points: &'a ContactPointsGene
}

impl<'a> SupportStructureInitializerSettings<'a> {
    pub fn new(settings: &'a Settings, contact_points: &'a ContactPointsGene) -> Self {
        Self {
            settings,
            contact_points
        }
    }
}

pub struct SupportStructureInitializer<'a> {
    settings: &'a Settings,
    contact_points: &'a ContactPointsGene
}

impl<'a> PopulationInitializer<CompressedSupportGene, SupportStructureInitializerSettings<'a>> for SupportStructureInitializer<'a> {
    fn new(settings: &SupportStructureInitializerSettings<'a>, _rand: crate::evolution::Random) -> Self {
        Self {
            settings: settings.settings,
            contact_points: settings.contact_points
        }
    }

    fn get_initial_individuals(&self) -> usize {
        self.settings.support_structure_optimization_settings.generation_size
    }

    fn get_random_individual(&self) -> CompressedSupportGene {
        todo!()
    }
}
