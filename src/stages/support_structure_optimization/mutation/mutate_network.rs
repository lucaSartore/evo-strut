use crate::{
    stages::support_structure_optimization::models::SupportStructureOptimizationGene,
    support::{neural_network::{NetworkMutationRates, NetworkMutationSettings}, random_distribution::RandomDistribution},
};

use super::*;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut SupportStructureOptimizationGene) {
    let rand = &mutator.rand;

    let valid_mutations = vec![
        NetworkMutationSettings::new(
            NetworkMutationRates::new(1.0, 0.0)
                .expect("invalid default contact-point grouping mutation rates"),
            RandomDistribution::Normal {
                mean: 0.,
                std_dev: 0.1,
            },
            RandomDistribution::InRange { low: -1., high: 1. },
        ),
        NetworkMutationSettings::new(
            NetworkMutationRates::new(0.05, 0.02)
                .expect("invalid default contact-point grouping mutation rates"),
            RandomDistribution::Normal {
                mean: 0.,
                std_dev: 0.1,
            },
            RandomDistribution::InRange { low: -1., high: 1. },
        ),
        NetworkMutationSettings::new(
            NetworkMutationRates::new(0.01, 0.15)
                .expect("invalid default contact-point grouping mutation rates"),
            RandomDistribution::Normal {
                mean: 0.,
                std_dev: 0.35,
            },
            RandomDistribution::InRange {
                low: -1.5,
                high: 1.5,
            },
        ),
    ];

    let mutation = rand
        .choose(&valid_mutations)
        .expect("there shall always be one mutation");

    gene.contact_radius.mutate(mutation, &mutator.rand);
}

