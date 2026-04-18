use smallvec::smallvec;

use crate::models::Point;
use crate::stages::support_structure_optimization::{ContactNode, MiddleNode, SupportNode, SupportNodeId};
use crate::support;

use super::*;
use super::super::models;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut SupportStructureGene) {
    let rand = &mutator.rand;
    let n1 = gene.random_non_base_node(rand);
    let n2 = gene.random_non_base_node(rand);
    if n1 == n2 {
        return;
    }

    let mut n1 = gene.get_gene(n1).clone();
    let mut n2 = gene.get_gene(n2).clone();

    if rand.random_choice(0.5) {
        n1.remove_random_support(rand);
    }
    if rand.random_choice(0.5) {
        n2.remove_random_support(rand);
    }

    let id = gene.new_random_id(rand);

    // selecting z of support
    let p1 = n1.get_position();
    let p2 = n2.get_position();

    let mut support_position = Point::random_in_between(p1, p2, rand);
    let mut z = p1.z.min(p2.z);
    if z > 0. {
        z = rand.next_f32(0., z);
    } else {
        z = 0.;
    }
    support_position.z = z;

    

    let new_node = MiddleNode {
        id,
        anchor: models::PositionAnchor { to: n1.id(), offset: support_position - p1},
        last_position: support_position,
        leans_on: smallvec![]
    };

    n1.add_support(new_node.id);
    n2.add_support(new_node.id);

    gene.nodes.insert(new_node.id, SupportNode::Middle(new_node));
    gene.nodes.insert(n1.id(), n1);
    gene.nodes.insert(n2.id(), n2);
}
