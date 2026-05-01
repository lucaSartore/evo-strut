use crate::stages::support_structure_optimization::models::SupportGroup;

use super::*;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut CompressedSupportGene) {
    let rand = &mutator.rand;
    let len = gene.groups.len();
    let id1 = rand.next_in_range_usize(0, len);
    let id2 = rand.next_in_range_usize(0, len);
    if id1 == id2 {
        return;
    }
    let (g1, g2) = gene.get_two_groups_mut(id1, id2);

    let mut new_group = SupportGroup {
        supports: vec![],
        layers: vec![]
    };
    new_group.supports.append(&mut g1.supports);
    new_group.supports.append(&mut g2.supports);

    let new_groups = regenerate_group::regenerate_group(mutator, &mut new_group);
    gene.add_groups(new_groups);
    gene.groups[id1] = new_group;
    gene.remove_group(id2);
}
