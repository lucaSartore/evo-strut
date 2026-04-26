use crate::{stages::support_structure_optimization::models::{LayerNode, SupportGroup, SupportLayer}};

use super::*;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut CompressedSupportGene) {
    let rand = &mutator.rand;
    let group = gene.rand_group_mut(rand);
    let max_height = group.max_height();
    let layer_height = rand.next_f32(0., max_height);

    add_layer(group, layer_height, mutator);
}

pub fn add_layer(group: &mut SupportGroup, layer_height: f32, mutator: &SupportStructureMutator) {
    let s = &mutator.settings.support_structure_optimization_settings;
    let rand = &mutator.rand;

    let (mut mean, mut covariance) = group.mean_and_cov(layer_height);

    covariance *= s.points_sampling_covariance_multiplier;
    mean.z = layer_height;

    
    let num_points_to_support = group.num_points_to_support_above(layer_height);
    let number_of_points_random = rand.next_distribution(&s.num_points_per_layer) as usize;
    
    let number_of_points = num_points_to_support.min(number_of_points_random);

    let layer = SupportLayer {
        center: mean,
        nodes: (0..number_of_points).map(|_| {
            LayerNode::new_random(&covariance, rand)
        }).collect()
    };

    group.layers.push(layer);
}
