use crate::{models::Plane, stages::support_structure_optimization::models::SupportGroup, support};

use super::*;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut CompressedSupportGene) {
    let rand = &mutator.rand;
    let group_id = gene.rand_group_id(rand);
    let group = &mut gene.groups[group_id];

    let p1 = group.random_support(rand);
    let p2 = group.random_support(rand);
    let p3 = group.random_support(rand);

    // can't split 
    let result = if p1 == p2 || p2 == p3 || p3 == p1 {
        split_random(rand, group)
    } else {
        let plane = Plane::from_points_and_max_distance(
            group.supports[p1].position,
            group.supports[p2].position,
            group.supports[p3].position
        );
        split_plane(plane, group)
    };

    let Some((mut g1, mut g2)) = result else { return };

    regenerate_group::regenerate_group(mutator, &mut g1);
    regenerate_group::regenerate_group(mutator, &mut g2);

    gene.groups[group_id] = g1;
    gene.groups.push(g2);
}

fn split_random(rand: &Random, group: &mut SupportGroup) -> Option<(SupportGroup, SupportGroup)> {
    let mut g1 = SupportGroup::empty();
    let mut g2 = SupportGroup::empty();

    for support in &group.supports {
        if rand.random_choice(0.5) {
            g1.supports.push(support.clone());
        } else {
            g2.supports.push(support.clone());
        }
    }

    if g1.is_empty() || g2.is_empty() {
        return None;
    }

    Some((g1, g2))
}
fn split_plane(plane: Plane, group: &mut SupportGroup) -> Option<(SupportGroup, SupportGroup)> {
    let mut g1 = SupportGroup::empty();
    let mut g2 = SupportGroup::empty();

    for support in &group.supports {
        if plane.classify_point(support.position) {
            g1.supports.push(support.clone());
        } else {
            g2.supports.push(support.clone());
        }
    }

    if g1.is_empty() || g2.is_empty() {
        return None;
    }

    Some((g1, g2))
}
