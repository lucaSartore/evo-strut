use crate::stages::support_structure_refinement::SupportNode;

use super::*;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut SupportStructureGene) {
    let rand = &mutator.rand;
    let n1 = gene.random_non_base_node(rand);
    let node = gene.nodes.get_mut(&n1).expect("node must be present");
    let leans_on = match node {
        SupportNode::Base(_) => panic!("node can't be a base one"),
        SupportNode::Middle(middle_node) => &mut middle_node.leans_on,
        SupportNode::Contact(contact_node) => &mut contact_node.leans_on,
    };

    if leans_on.len() == 0 {
        return;
    }

    let index = rand.next_in_range(0, leans_on.len() as u64);

    leans_on.swap_remove(index as usize);
}
