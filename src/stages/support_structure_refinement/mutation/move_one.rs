
use crate::models::Point;
use crate::stages::support_structure_refinement::SupportNode;

use super::*;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut SupportStructureGene) {
    let rand = &mutator.rand;
    let Some(n1) = gene.random_middle_node(rand) else {
        return;
    };
    let SupportNode::Middle(node) = gene.nodes.get_mut(&n1)
        .expect("node must be present") else {
        panic!("node must be middle");
    };

    let std = mutator.settings.support_structure_optimization_settings.node_position_mutation_std;
    let new_anchor_offset = Point::random(node.anchor.offset, std, rand);
    node.anchor.offset = new_anchor_offset
}
