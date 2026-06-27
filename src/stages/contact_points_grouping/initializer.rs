use crate::{
    evolution::{PopulationInitializer, Random},
    models::Settings,
    stages::contact_points_grouping::models::ContactPointGroupingGene,
    support::neural_network::NeuralNetwork,
};

pub struct ContactPointGroupingInitializerSettings<'a> {
    settings: &'a Settings,
}

impl<'a> ContactPointGroupingInitializerSettings<'a> {
    pub fn new(settings: &'a Settings) -> Self {
        Self { settings }
    }
}

pub struct ContactPointGroupingInitializer<'a> {
    settings: &'a Settings,
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
