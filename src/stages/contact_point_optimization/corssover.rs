use crate::models::Settings;
use crate::evolution::*;
use super::models::ContactPointsGene;
use rand::Rng;

pub struct ContactPointCrossover{
    settings: Settings,
    rand: Random
}

impl Crossover<ContactPointsGene, Settings> for ContactPointCrossover {
    fn new(settings: &Settings, rand: Random) -> Self {
        Self {
            settings: settings.clone(),
            rand
        }
    }

    fn crossover(&self, a: &ContactPointsGene, b: &ContactPointsGene) -> ContactPointsGene {
        todo!()
    }
}
