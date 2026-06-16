use hashbrown::HashMap;
use itertools::Itertools;
use rerun::external::re_sdk_types::impl_into_cow;

use crate::{evolution::Cost, models::Point};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Node {
    pub id: usize,
    pub point: Point,
    pub father: usize,
}

pub struct Tree {
    // nodes, 0 is the root
    pub nodes: Vec<Node>,
    pub pos_to_id: HashMap<Point, usize>,
}

impl Tree {
    pub fn new(root: Point) -> Self {
        let root_node = Node {
            id: 0,
            point: root,
            father: 0,
        };

        let mut pos_to_id = HashMap::new();
        pos_to_id.insert(root, root_node.id);

        Self {
            nodes: vec![root_node],
            pos_to_id,
        }
    }

    pub fn iter_branches(&self) -> impl Iterator<Item = (Point, Point)> {
        self.nodes
            .iter()
            .skip(1) // skipping the route
            .map(|x| (x.point, self.nodes[x.father].point))
            // ordering by the height of the node
            .sorted_by_key(|x| Cost::new(x.0.z))
    }

    // add a new node as a leaf
    pub fn add_node(&mut self, position: Point, father: usize) -> usize {
        if let Some(id) = self.pos_to_id.get(&position) {
            return *id;
        }

        let id = self.nodes.len();
        self.nodes.push(Node {
            id,
            point: position,
            father,
        });
        self.pos_to_id.insert(position, id);
        id
    }

    // split a branch, by putting a new node in the middle
    pub fn split_branch(&mut self, father: usize, son: usize, new_node: Point) -> usize {
        debug_assert_eq!(self.nodes[son].father, father);

        if self.nodes[father].point == new_node {
            return father;
        }

        if self.nodes[son].point == new_node {
            return son;
        }

        if let Some(id) = self.pos_to_id.get(&new_node) {
            self.nodes[son].father = *id;
            return *id;
        }

        let id = self.add_node(new_node, father);
        self.nodes[son].father = id;
        id
    }
}
