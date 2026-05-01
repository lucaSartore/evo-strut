use crate::{models::Point, stages::support_structure_optimization::models::{LayerConnections, LayerNode}};

use super::*;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut SupportGroup) {
    let rand = &mutator.rand;
    let Some(layer_id) = gene.random_layer_id(rand) else { return };
    let (_, cov) = gene.mean_and_cov_layer(layer_id);
    let layer = &mut gene.layers[layer_id];
    let offset = Point::random_zero_z(Point::ZERO, &cov, rand);
    layer.nodes.push(LayerNode {
        offset,
        connections: LayerConnections::new_random(rand)
    });
}
