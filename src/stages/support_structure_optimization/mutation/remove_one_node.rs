use smallvec::smallvec;

use crate::models::Point;
use crate::stages::support_structure_optimization::{ContactNode, MiddleNode, SupportNode};
use crate::support;
use crate::support::remove_random::RemoveRandom;

use super::*;
use super::super::models;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut SupportStructureGene) {
    let rand = &mutator.rand;
    let n1 = gene.random_non_contact_node(rand);
    gene.nodes.remove(&n1);
}
