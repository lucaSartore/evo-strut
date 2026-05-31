use crate::{
    models::Point,
    stages::support_structure_optimization::models::SupportStructureOptimizationGene,
    support::random_distribution::RandomDistribution,
};

use super::*;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut SupportStructureOptimizationGene) {
    let rand = &mutator.rand;
    let mutation_probability = rand.next_distribution(&RandomDistribution::InRange { low: 0.05, high: 0.4 });
    let mutation_std = rand.next_distribution(&RandomDistribution::InRange { low: 0.5, high: 5. });

    for p in gene.supports.iter_mut() {
        if rand.random_choice(mutation_probability) {
            p.position = Point::random(p.position, mutation_std, rand);
        }
    }
}

