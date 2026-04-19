use smallvec::smallvec;

use crate::models::Point;
use crate::stages::support_structure_optimization::{ContactNode, MiddleNode, PositionAnchor, SupportNode};
use crate::support;
use crate::support::remove_random::RemoveRandom;

use super::*;
use super::super::models;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut SupportStructureGene) {
    let rand = &mutator.rand;
    let n1 = gene.random_contact_node(rand);
    let new_node_id = gene.new_random_id(rand);

    let SupportNode::Contact(node) = gene.nodes.get_mut(&n1)
        .expect("node must be found") else { panic!("node must be contact")};

    let to_remove = rand.next_in_range(0, node.leans_on.len() as u64) as usize;

    let old_support_id = node.leans_on[to_remove];
    node.leans_on[to_remove] = new_node_id;

    let node_position = node.position;
    let old_support = gene.nodes.get(&old_support_id).expect("support must be found");
    let old_support_position = old_support.get_position();
    let new_node_position = Point::random_in_between(node_position, old_support_position, rand);

    let new_node = MiddleNode {
        id: new_node_id,
        anchor: PositionAnchor::new(n1, node_position, new_node_position),
        last_position: new_node_position,
        leans_on: smallvec![old_support_id]
    };

    gene.nodes.insert(new_node_id, SupportNode::Middle(new_node));
}
