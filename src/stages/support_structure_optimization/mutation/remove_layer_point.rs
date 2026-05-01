use super::*;

pub fn mutate(mutator: &SupportStructureMutator, group: &mut SupportGroup) {
    let s = &mutator.settings.support_structure_optimization_settings;
    let rand = &mutator.rand;

    let removable_layers: Vec<usize> = group
        .layers
        .iter()
        .enumerate()
        .filter(|(_, layer)| layer.nodes.len() > s.min_points_in_layer)
        .map(|(index, _)| index)
        .collect();

    let Some(layer_id) = rand.choose(&removable_layers) else {
        return;
    };
    let layer = &mut group.layers[*layer_id];
    let num_points = layer.nodes.len();
    if num_points == 1 {
        return;
    }
    let node_id = rand.next_in_range_usize(0, num_points);
    layer.nodes.swap_remove(node_id);
}
