use super::ContactPointGroupingGene;
use crate::{
    evolution::{Crossover, Random},
    models::Settings,
};

pub struct ContactPointsGroupingSettings<'a> {
    settings: &'a Settings,
}

impl<'a> ContactPointsGroupingSettings<'a> {
    pub fn new(settings: &'a Settings) -> Self {
        Self { settings }
    }
}

pub struct ContactPointGroupingCrossover<'a> {
    settings: &'a Settings,
    rand: Random,
}

impl<'a> Crossover<ContactPointGroupingGene, ContactPointsGroupingSettings<'a>>
    for ContactPointGroupingCrossover<'a>
{
    fn new(settings: &ContactPointsGroupingSettings<'a>, rand: Random) -> Self {
        Self {
            settings: settings.settings,
            rand,
        }
    }

    fn crossover(
        &self,
        a: &ContactPointGroupingGene,
        b: &ContactPointGroupingGene,
    ) -> ContactPointGroupingGene {
        let strategy = self
            .rand
            .choose(
                &self
                    .settings
                    .contact_points_grouping_settings
                    .valid_crossovers,
            )
            .expect("at least one valid crossover strategy should be provided");
        let new_network = a
            .network
            .crossover(&b.network, strategy, &self.rand)
            .expect("crossover fail");
        ContactPointGroupingGene {
            network: new_network,
        }
    }
}
