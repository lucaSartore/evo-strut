use crate::{
    evolution::{Mutator, Random},
    models::Settings,
    stages::contact_points_grouping::models::ContactPointGroupingGene,
};

pub struct ContactPointGroupingMutatorSettings<'a> {
    settings: &'a Settings,
}

impl<'a> ContactPointGroupingMutatorSettings<'a> {
    pub fn new(settings: &'a Settings) -> Self {
        Self { settings }
    }
}

pub struct ContactPointGroupingMutator<'a> {
    pub settings: &'a Settings,
    pub rand: Random,
}

impl<'a> Mutator<ContactPointGroupingGene, ContactPointGroupingMutatorSettings<'a>>
    for ContactPointGroupingMutator<'a>
{
    fn new(settings: &ContactPointGroupingMutatorSettings<'a>, rand: Random) -> Self {
        Self {
            settings: settings.settings,
            rand,
        }
    }

    fn mutate(&self, gene: &mut ContactPointGroupingGene) {
        let strategy = self
            .rand
            .choose(
                &self
                    .settings
                    .contact_points_grouping_settings
                    .valid_mutations,
            )
            .expect("at least one valid mutation strategy should be provided");
        gene.network
            .mutate(strategy, &self.rand)
            .expect("contact-point grouping network mutation failed");
    }
}
