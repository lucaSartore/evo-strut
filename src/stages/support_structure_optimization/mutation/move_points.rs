use crate::{
    models::Point,
    stages::support_structure_optimization::models::SupportStructureOptimizationGene,
    support::random_distribution::RandomDistribution,
};

use super::*;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut SupportStructureOptimizationGene) {
    let rand = &mutator.rand;
    let s = &mutator.settings.support_structure_optimization_settings;

    let sp = &s.point_mutation_probability_range;
    let mutation_probability = rand.next_distribution(&RandomDistribution::InRange {
        low: sp.0,
        high: sp.1
    });

    let sr = &s.point_mutation_std_range;
    let mutation_std = rand.next_distribution(&RandomDistribution::InRange { low: sr.0, high: sr.1 });

    for p in gene.supports.iter_mut() {
        if rand.random_choice(mutation_probability) {
            p.position = Point::random(p.position, mutation_std, rand);
        }
    }
}
