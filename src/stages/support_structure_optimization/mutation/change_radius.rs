use crate::stages::support_structure_optimization::models::SupportStructureOptimizationGene;

use super::*;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut SupportStructureOptimizationGene) {
    let rand = &mutator.rand;

    let Some(point) = gene.random_support_mut(rand) else { return };

    // todo: hardcoded values
    let multiplier = rand.next_f32(0.8, 1.3);
    point.radius = (point.radius * multiplier).clamp(1., 5.)
}

