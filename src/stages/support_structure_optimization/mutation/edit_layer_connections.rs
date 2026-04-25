use super::*;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut CompressedSupportGene) {
    let rand = &mutator.rand;
    let group = gene.rand_group_mut(rand);
    let Some(layer) = group.random_layer_mut(rand) else { return };
    let mutations = rand.next_in_range(1, 3);
    for _ in 0..mutations {
        layer.mutate_random_connection(rand);
    }
}
