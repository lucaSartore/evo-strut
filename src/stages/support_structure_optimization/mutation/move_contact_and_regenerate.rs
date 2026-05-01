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

    let element = g1.pop_random_support(rand);
    g2.add_support(element);

    let new_groups_2 = regenerate_group::regenerate_group(mutator, g2);
    if g1.is_empty() {
        gene.remove_group(id1);
    } else {
        let new_groups_1 = regenerate_group::regenerate_group(mutator, g1);
        gene.add_groups(new_groups_1);
    }
    gene.add_groups(new_groups_2);
}
