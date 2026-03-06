use crate::models::Settings;
use crate::evolution::*;
use super::models::ContactPointsGene;
use rand::Rng;

pub struct ContactPointCrossover{
}

impl Crossover<ContactPointsGene, Settings> for ContactPointCrossover {
    fn new(settings: &Settings, rand: Random) -> Self {
        todo!()
    }

    fn crossover(&self, a: &ContactPointsGene, b: &ContactPointsGene) -> ContactPointsGene {
        todo!()
    }
}
