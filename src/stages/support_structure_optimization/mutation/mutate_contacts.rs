use crate::stages::support_structure_optimization::models::SupportStructureOptimizationGene;

use super::*;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut SupportStructureOptimizationGene) {
    let rand = &mutator.rand;

    let Some(point) = gene.random_support_mut(rand) else { return };

    let to_sum = *rand.choose(&[-1, 1]).expect("can't be empty");
    point.num_contacts = (point.num_contacts as i32 + to_sum).clamp(0, 3) as u32;
}

