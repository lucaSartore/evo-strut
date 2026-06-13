use crate::{
    models::Point, stages::support_structure_optimization::models::SupportStructureOptimizationGene, support::random_distribution::RandomDistribution
};

use super::*;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut SupportStructureOptimizationGene) {
    let rand = &mutator.rand;

    let p = gene.random_support_mut(rand)
        .expect("can't be empty");

    // todo: hardcoded value
    let mutation_std = rand.next_distribution(&RandomDistribution::InRange { low: 0.5, high: 5. });
    p.position = Point::random(p.position, mutation_std, rand);
}

