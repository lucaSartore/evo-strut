use crate::stages::support_structure_optimization::models::SupportStructureOptimizationGene;

use super::*;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut SupportStructureOptimizationGene) {
    let len = gene.supports.len();
    if len == 0 {
        return
    }
    let to_remove = mutator.rand.next_in_range(0, len as u64) as usize;
    gene.supports.swap_remove(to_remove);
}
