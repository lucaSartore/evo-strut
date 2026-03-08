use crate::{evolution::{Evaluator, PopulationInitializer, Random}, models::{Settings, SurfaceGraph, TriangleId}, stages::contact_point_optimization::models::ContactPointsGene};

pub struct  ContactPointInitializer {
    settings: Settings,
    rand: Random
}

impl PopulationInitializer<ContactPointsGene, Settings> for ContactPointInitializer {
    fn new(settings: &Settings, rand: Random) -> Self {
        Self {
            settings: settings.clone(),
            rand
        }
    }

    fn get_initial_individuals(&self) -> usize {
        return 10;
        todo!()
    }

    fn get_random_individual(&self) -> ContactPointsGene {
        return ContactPointsGene::default();
        todo!()
    }
}
