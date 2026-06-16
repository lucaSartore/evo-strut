use std::f32::consts::PI;

use crate::{evolution::Cost, models::Point};

mod tree;
pub use tree::Tree;

pub struct TreeGenerator {
    root: Point,
    leaves: Vec<Point>,
    min_horizon_angle: f32,
    interpolation_step_size: f32,
}

impl TreeGenerator {
    pub fn new<I>(
        root: Point,
        leaves: I,
        // maximum angle allowed in the tree (in degrees)
        min_horizon_angle: f32,
        interpolation_step_size: f32,
    ) -> Self
    where
        I: IntoIterator<Item = Point>,
    {
        Self {
            root,
            leaves: leaves.into_iter().collect(),
            min_horizon_angle: min_horizon_angle * PI / 180.0,
            interpolation_step_size,
        }
    }

    pub fn run(&self) -> Tree {
        // calculate the center of the leaf
        // order the leafs from the closest to the center, to the furthest
        // (without considering the Z dimension in the distance function)
        let center = self.leaf_center_xy();
        let mut leaves = self.leaves.clone();
        // order so that we first add the closest node to the center
        // and then we start from the furthest one, and we go toward
        // the center
        leaves.sort_by_key(|a| Cost::new(-xy_distance_sq(*a, center)));
        let len = leaves.len();
        if len >= 2 {
            leaves.swap(0, len - 1);
        }

        let mut tree = Tree::new(self.root);

        for leaf in leaves {
            self.insert_leaf(&mut tree, leaf);
        }

        tree
    }

    fn insert_leaf(&self, tree: &mut Tree, leaf: Point) {
        let best = self.best_connection_point(tree, leaf);
        let father = match best {
            Some(ConnectionCandidate::Node { node }) => node,
            Some(ConnectionCandidate::Branch { father, son, point }) => {
                tree.split_branch(father, son, point)
            }
            None => 0,
        };

        tree.add_node(leaf, father);
    }

    fn leaf_center_xy(&self) -> Point {
        if self.leaves.is_empty() {
            return self.root;
        }

        let sum = self.leaves.iter().fold(Point::ZERO, |acc, point| {
            acc + Point::new(point.x, point.y, 0.0)
        });

        sum.to_scaled(1.0 / self.leaves.len() as f32)
    }

    fn best_connection_point(&self, tree: &Tree, leaf: Point) -> Option<ConnectionCandidate> {
        let mut best: Option<(f32, ConnectionCandidate)> = None;

        for node in &tree.nodes {
            self.try_candidate(
                &mut best,
                node.point,
                ConnectionCandidate::Node { node: node.id },
                leaf,
            );
        }

        for son in tree.nodes.iter().filter(|node| node.id != 0) {
            let father = &tree.nodes[son.father];
            let points = Point::interpolate(father.point, son.point, self.interpolation_step_size);
            for point in points.iter().take(points.len() - 1).skip(1) {
                let candidate = ConnectionCandidate::Branch {
                    father: father.id,
                    son: son.id,
                    point: *point,
                };
                self.try_candidate(&mut best, *point, candidate, leaf);
            }
        }

        best.map(|(_, candidate)| candidate)
    }

    fn try_candidate(
        &self,
        best: &mut Option<(f32, ConnectionCandidate)>,
        point: Point,
        candidate: ConnectionCandidate,
        leaf: Point,
    ) {
        if Point::horizon_angle(point, leaf) < self.min_horizon_angle {
            return;
        }

        let distance = (leaf - point).abs();
        if best
            .as_ref()
            .is_none_or(|(best_distance, _)| distance < *best_distance)
        {
            *best = Some((distance, candidate));
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum ConnectionCandidate {
    Node {
        node: usize,
    },
    Branch {
        father: usize,
        son: usize,
        point: Point,
    },
}

fn xy_distance_sq(a: Point, b: Point) -> f32 {
    (a.x - b.x).powi(2) + (a.y - b.y).powi(2)
}
