use crate::stages::support_structure_optimization::SupportNode;

use super::*;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut SupportStructureGene) {
    let rand = &mutator.rand;
    let n1 = gene.random_non_base_node(rand);
    let Some(node_to_add) = gene.random_middle_node(rand) else {
        return;
    };
    if node_to_add == n1 {
        return;
    }

    let node = gene.nodes.get_mut(&n1).expect("node must be present");
    let leans_on = match node {
        SupportNode::Base(_) => panic!("node can't be a base one"),
        SupportNode::Middle(middle_node)  => &mut middle_node.leans_on,
        SupportNode::Contact(contact_node) => &mut contact_node.leans_on,
    };

    // 50% probability to remove the node (instead of just adding one)
    if rand.random_choice(0.5) {
        let to_remove = rand.next_in_range(0, leans_on.len() as u64) as usize;
        leans_on.swap_remove(to_remove);
    }

    if leans_on.contains(&node_to_add) {
        return;
    }

    leans_on.push(node_to_add);
}
