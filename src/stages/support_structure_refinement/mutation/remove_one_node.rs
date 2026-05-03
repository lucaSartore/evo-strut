use super::*;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut SupportStructureGene) {
    let rand = &mutator.rand;
    let n1 = gene.random_non_contact_node(rand);
    gene.nodes.remove(&n1);
}
