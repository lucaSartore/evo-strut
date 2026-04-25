use crate::{models::Point, stages::support_structure_optimization::models::{LayerConnections, LayerNode}};

use super::*;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut CompressedSupportGene) {
    let rand = &mutator.rand;
    let group = gene.rand_group_mut(rand);
    let Some(layer_id) = group.random_layer_id(rand) else { return };
    let (_, cov) = group.mean_and_cov_layer(layer_id);
    let layer = &mut group.layers[layer_id];
    let offset = Point::random_zero_z(Point::ZERO, &cov, rand);
    layer.nodes.push(LayerNode {
        offset,
        connections: LayerConnections::new_random(rand)
    });
}
