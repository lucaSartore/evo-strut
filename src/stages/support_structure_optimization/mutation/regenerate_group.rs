use crate::stages::support_structure_optimization::models::SupportGroup;

use super::*;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut CompressedSupportGene) {
    let rand = &mutator.rand;
    let group = gene.rand_group_mut(rand);
    regenerate_group(mutator, group);
}

pub fn regenerate_group(mutator: &SupportStructureMutator, group: &mut SupportGroup) {
    let s = &mutator.settings.support_structure_optimization_settings;
    let rand = &mutator.rand;
    let layer_density = rand.next_distribution(&s.layer_density);
    let max_height = group.max_height();
    let num_layers = (max_height * layer_density) as usize;


    // reset all layers
    group.layers = vec![];

    // recreate all layers
    for i in (1..num_layers).rev() {
        let height = i as f32 / num_layers as f32 * max_height;
        super::add_layer::add_layer(group, height, mutator);
    }

    // we start with all connections set (to reduce variability between different generation of
    // groups, so that the underlying mutation can have a bigger impact)
    group.set_all_connections();
}
