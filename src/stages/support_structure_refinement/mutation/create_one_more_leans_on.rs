use smallvec::smallvec;

use crate::{
    models::Point,
    stages::support_structure_refinement::{MiddleNode, PositionAnchor, SupportNode},
};

use super::*;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut SupportStructureGene) {
    let rand = &mutator.rand;
    let n1 = gene.random_non_base_node(rand);

    let new_node_id = gene.new_random_id(rand);

    let node = gene.nodes.get_mut(&n1).expect("node must be present");

    let position = node.get_position();
    let mut position_zero = position;
    position_zero.z = 0.;

    let mut new_node_position = Point::random_in_between(position, position_zero, rand);
    new_node_position = Point::random(
        new_node_position,
        mutator
            .settings
            .support_structure_refinement_settings
            .node_position_mutation_std,
        rand,
    );

    let leans_on = match node {
        SupportNode::Base(_) => panic!("node can't be a base one"),
        SupportNode::Middle(middle_node) => &mut middle_node.leans_on,
        SupportNode::Contact(contact_node) => &mut contact_node.leans_on,
    };

    leans_on.push(new_node_id);

    gene.nodes.insert(
        new_node_id,
        SupportNode::Middle(MiddleNode {
            id: new_node_id,
            anchor: PositionAnchor::new(n1, position, new_node_position),
            last_position: new_node_position,
            leans_on: smallvec![],
        }),
    );
}
