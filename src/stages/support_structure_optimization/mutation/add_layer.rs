use super::*;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut CompressedSupportGene) {
    let rand = &mutator.rand;
    let group = gene.rand_group_mut(rand);
    let max_height = group.max_height();
    let layer_height = rand.next_f32(0., max_height);
    let (mean, covariance) = group.mean_and_cov(layer_height);
}
