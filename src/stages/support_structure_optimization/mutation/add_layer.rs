use crate::stages::support_structure_optimization::models::{LayerNode, SupportLayer};

use super::*;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut CompressedSupportGene) {
    let s = &mutator.settings.support_structure_optimization_settings;
    let rand = &mutator.rand;
    let group = gene.rand_group_mut(rand);
    let max_height = group.max_height();
    let layer_height = rand.next_f32(0., max_height);
    let (mut mean, mut covariance) = group.mean_and_cov(layer_height);

    covariance *= s.points_sampling_covariance_multiplier;
    mean.z = layer_height;

    let number_of_points = rand.next_distribution(&s.num_points_per_layer) as usize;

    let layer = SupportLayer {
        center: mean,
        nodes: (0..number_of_points).map(|_| {
            LayerNode::new_random(mean, &covariance, rand)
        }).collect()
    };

    group.layers.push(layer);
}
