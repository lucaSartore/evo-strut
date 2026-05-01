use crate::{
    evolution::{Mutator, Random},
    models::{Settings, SurfaceGraph},
    stages::contact_points_grouping::models::ContactPointGroupingGene,
};

pub struct ContactPointGroupingMutatorSettings<'a> {
    settings: &'a Settings,
    graph: &'a SurfaceGraph,
}

impl<'a> ContactPointGroupingMutatorSettings<'a> {
    pub fn new(settings: &'a Settings, graph: &'a SurfaceGraph) -> Self {
        Self { settings, graph }
    }
}

pub struct ContactPointGroupingMutator<'a> {
    pub settings: &'a Settings,
    pub graph: &'a SurfaceGraph,
    pub rand: Random,
}

impl<'a> Mutator<ContactPointGroupingGene, ContactPointGroupingMutatorSettings<'a>>
    for ContactPointGroupingMutator<'a>
{
    fn new(settings: &ContactPointGroupingMutatorSettings<'a>, rand: Random) -> Self {
        Self {
            settings: settings.settings,
            graph: settings.graph,
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
