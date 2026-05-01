use crate::support::random_distribution::RandomDistribution;

use super::*;

pub fn mutate(mutator: &SupportStructureMutator, group: &mut SupportGroup) {
    let s = &mutator.settings.support_structure_optimization_settings;
    let rand = &mutator.rand;
    let max_height = group.max_height();
    let Some(layer) = group.random_layer_mut(rand) else {
        return;
    };
    let distribution = RandomDistribution::Normal {
        mean: 0.,
        std_dev: s.layer_height_motion_std,
    };
    let motion = rand.next_distribution(&distribution);

    layer.center.z = (layer.center.z + motion).clamp(0., max_height);
}
