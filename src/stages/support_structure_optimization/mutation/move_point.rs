use crate::{
    models::Point,
    stages::support_structure_optimization::models::SupportStructureOptimizationGene,
    support::random_distribution::RandomDistribution,
};

use super::*;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut SupportStructureOptimizationGene) {
    let rand = &mutator.rand;
    let s = &mutator.settings.support_structure_optimization_settings.point_mutation_std_range;

    let Some(p) = gene.random_support_mut(rand) else { return };

    let mutation_std = rand.next_distribution(&RandomDistribution::InRange { low: s.0, high: s.1 });
    p.position = Point::random(p.position, mutation_std, rand);
}
