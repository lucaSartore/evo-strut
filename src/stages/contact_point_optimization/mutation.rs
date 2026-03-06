use crate::{evolution::{Mutator, Random}, models::Settings};
use super::models::ContactPointsGene;


pub struct ContactPointMutator {
}

impl Mutator<ContactPointsGene, Settings> for ContactPointMutator {
    fn new(settings: &Settings, rand: Random) -> Self {
        todo!()
    }

    fn mutate(&self, gene: ContactPointsGene) -> ContactPointsGene {
        todo!()
    }
}
