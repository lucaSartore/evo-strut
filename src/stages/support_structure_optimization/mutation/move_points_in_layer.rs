use crate::{models::Point, support::random_distribution::RandomDistribution};

use super::*;

pub fn mutate(mutator: &SupportStructureMutator, group: &mut SupportGroup) {
    let s = &mutator.settings.support_structure_optimization_settings;
    let rand = &mutator.rand;
    let Some(layer_id) = group.random_layer_id(rand) else {
        return;
    };
    let layer = &mut group.layers[layer_id];
    let Some(node) = rand.choose_mut(&mut layer.nodes) else {
        return;
    };
    let distribution = RandomDistribution::Normal {
        mean: 0.,
        std_dev: s.layer_point_motion_std,
    };
    let motion = Point::new(
        rand.next_distribution(&distribution),
        rand.next_distribution(&distribution),
        0.,
    );

    node.offset = node.offset + motion;
}
