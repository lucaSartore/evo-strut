use crate::{evolution::{Mutator, Random}, models::Settings};
use super::models::ContactPointsGene;


pub struct ContactPointMutator {
    settings: Settings,
    rand: Random
}

impl Mutator<ContactPointsGene, Settings> for ContactPointMutator {
    fn new(settings: &Settings, rand: Random) -> Self {
        Self {
            settings: settings.clone(),
            rand
        }
    }

    fn mutate(&self, gene: ContactPointsGene) -> ContactPointsGene {
        todo!()
    }
}
