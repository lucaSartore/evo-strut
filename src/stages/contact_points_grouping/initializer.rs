use rerun::external::glam::usize;

use crate::{
    evolution::{PopulationInitializer, Random},
    models::{Settings, SurfaceGraph},
    stages::{
        contact_point_optimization::ContactPointsGene,
        contact_points_grouping::models::ContactPointGroupingGene,
    },
    support::neural_network::NeuralNetwork,
};

pub struct ContactPointGroupingInitializerSettings<'a> {
    settings: &'a Settings,
    contact_points: &'a ContactPointsGene,
    graph: &'a SurfaceGraph,
}

impl<'a> ContactPointGroupingInitializerSettings<'a> {
    pub fn new(
        settings: &'a Settings,
        contact_points: &'a ContactPointsGene,
        graph: &'a SurfaceGraph,
    ) -> Self {
        Self {
            settings,
            contact_points,
            graph,
        }
    }
}

pub struct ContactPointGroupingInitializer<'a> {
    settings: &'a Settings,
    contact_points: &'a ContactPointsGene,
    graph: &'a SurfaceGraph,
    rand: Random,
}

impl<'a>
    PopulationInitializer<ContactPointGroupingGene, ContactPointGroupingInitializerSettings<'a>>
    for ContactPointGroupingInitializer<'a>
{
    fn new(
        settings: &ContactPointGroupingInitializerSettings<'a>,
        rand: crate::evolution::Random,
    ) -> Self {
        Self {
            settings: settings.settings,
            contact_points: settings.contact_points,
            graph: settings.graph,
            rand,
        }
    }

    fn get_initial_individuals(&self) -> usize {
        self.settings
            .support_structure_optimization_settings
            .generation_size
    }

    fn get_random_individual(&self) -> ContactPointGroupingGene {
        let settings = &self.settings.contact_points_grouping_settings;
        let network = NeuralNetwork::random(
            settings.network_topology.clone(),
            settings.network_weight_initialization,
            &self.rand,
        )
        .expect("failed to initialize contact-point grouping neural network");
        ContactPointGroupingGene::new(network)
    }
}
