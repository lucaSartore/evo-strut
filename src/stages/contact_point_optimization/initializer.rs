use crate::{evolution::{Evaluator, PopulationInitializer, Random}, models::{Settings, SurfaceGraph, TriangleId}, stages::contact_point_optimization::models::ContactPointsGene};

pub struct  ContactPointInitializer {
}

impl PopulationInitializer<ContactPointsGene, Settings> for ContactPointInitializer {
    fn new(settings: &Settings, rand: Random) -> Self {
        todo!()
    }

    fn get_initial_individuals(&self) -> usize {
        todo!()
    }

    fn get_random_individual(&self) -> ContactPointsGene {
        todo!()
    }
}
