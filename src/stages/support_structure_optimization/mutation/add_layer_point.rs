use crate::{models::Point, stages::support_structure_optimization::models::{LayerConnections, LayerNode}};

use super::*;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut SupportGroup) {
    let s = &mutator.settings.support_structure_optimization_settings;
    let rand = &mutator.rand;
    let Some(layer_id) = gene.random_layer_id(rand) else { return };
    let layer = &gene.layers[layer_id];
    let points = gene.points_to_support_above(layer.center.z);
    if points.is_empty() {
        return;
    }
    let layer = &mut gene.layers[layer_id];
    let offset = layer.random_point_in_self(&points, None, rand, s.layer_node_creation_update_step);
    layer.nodes.push(LayerNode {
        offset,
        connections: LayerConnections::new_random(rand)
    });
}
