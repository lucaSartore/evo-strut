use std::collections::BinaryHeap;

use hashbrown::HashMap;
use smallvec::{smallvec, SmallVec};

use crate::{
    evolution::Random,
    models::Point,
    stages::{
        criticality_detection::propagation::QueuedElement,
        support_structure_optimization::SupportNodeId,
    },
};

#[derive(Clone, Debug)]
pub struct Neighbor {
    pub id: SupportNodeId,
    pub distance: f32,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: SupportNodeId,
    pub position: Point,
    pub visited: bool,
    pub distance_from_source: f32,
    pub neighbors: SmallVec<[Neighbor; 4]>,
    pub supported: bool,
    pub radius: f32,
}

#[derive(Debug, Clone)]
pub struct StructureGraph {
    pub nodes: HashMap<SupportNodeId, Node>,
}

impl StructureGraph {
    pub fn new() -> StructureGraph {
        StructureGraph {
            nodes: HashMap::default(),
        }
    }

    pub fn new_random_id(&self, rand: &Random) -> SupportNodeId {
        let id = SupportNodeId(rand.next_u32());
        // re-generate it, as it is already taken
        if self.nodes.contains_key(&id) {
            return self.new_random_id(rand);
        }
        id
    }

    #[allow(dead_code)]
    pub fn reset_all_nodes(&mut self) {
        for (_, n) in self.nodes.iter_mut() {
            n.visited = false;
            n.distance_from_source = 0.
        }
    }

    pub fn reset_some_nodes(&mut self, to_reset: &[SupportNodeId]) {
        for n_id in to_reset {
            let Some(n) = self.nodes.get_mut(n_id) else {
                continue;
            };
            n.visited = false;
            n.distance_from_source = 0.;
        }
    }

    /// visit all the nodes from a source and set the distances.
    /// then return the visited nodes from the closest to the furthest
    pub fn mark_distances(&mut self, source: SupportNodeId) -> Vec<SupportNodeId> {
        let mut visited = vec![];
        let mut queue = BinaryHeap::new();
        queue.push(QueuedElement::new_from_value(source, 0.0));
        while let Some(e) = queue.pop() {
            let distance = e.value.as_f32();
            let id = e.id;

            let node = self.nodes.get_mut(&id).expect("node id not found");
            if node.visited {
                continue;
            }

            visited.push(id);
            node.visited = true;
            node.distance_from_source = distance;

            let neighbors = node.neighbors.clone();
            for neighbor in neighbors {
                if self.nodes[&neighbor.id].visited {
                    continue;
                }
                queue.push(QueuedElement::new_from_value(
                    neighbor.id,
                    distance + neighbor.distance,
                ));
            }
        }
        visited
    }

    /// returns the IDs of the nodes supporting the current one, including the support weights
    /// (used to not duplicate stiffness in case of cyclical structures)
    pub fn get_weighted_supports(&self, id: SupportNodeId) -> SmallVec<[(SupportNodeId, f32); 4]> {
        let this = &self.nodes[&id];
        let mut to_return = smallvec![];

        for supporter in self.get_supports(this) {
            let weighted_supports: SmallVec<[_; 4]> = self
                .get_supported(supporter)
                // inverting the distance, so that closer nodes
                // have an higher weight
                .map(|x| (x.0, 1.0 / x.1))
                .collect();

            let weight_this = weighted_supports
                .iter()
                .find(|x| x.0.id == id)
                .expect("id of current node must be found")
                .1;

            let weight_total: f32 = weighted_supports.iter().map(|x| x.1).sum();

            // the support provided a node X, is shared
            // by all the nodes supported by X, and is shared
            // in a way that makes it inversely proportional to the distance
            // a node has with the root node
            to_return.push((supporter.id, weight_this / weight_total));
        }
        to_return
    }

    /// return all the nodes that support a specific node
    fn get_supports(&self, node: &Node) -> impl Iterator<Item = &Node> {
        node.neighbors
            .iter()
            .map(|x| &self.nodes[&x.id])
            .filter(|x| x.distance_from_source > node.distance_from_source)
    }

    /// return all the nodes that are supported by a specific node
    /// including the distance of the support
    fn get_supported(&self, node: &Node) -> impl Iterator<Item = (&Node, f32)> {
        node.neighbors
            .iter()
            .map(|x| {
                let n = &self.nodes[&x.id];
                (n, n.distance_from_source + x.distance)
            })
            .filter(|x| x.0.distance_from_source < node.distance_from_source)
    }

    pub fn add_node(
        &mut self,
        id: SupportNodeId,
        position: Point,
        neighbors: &[SupportNodeId],
        supported: bool,
        radius: f32,
    ) {
        let mut new_node = Node {
            id,
            position,
            visited: false,
            distance_from_source: 0.,
            neighbors: smallvec![],
            supported,
            radius,
        };

        for n_id in neighbors {
            let Some(n) = self.nodes.get_mut(n_id) else {
                continue;
            };
            let distance = (position - n.position).abs();
            n.neighbors.push(Neighbor { id, distance });
            new_node.neighbors.push(Neighbor {
                id: *n_id,
                distance,
            });
        }
        let old_node = self.nodes.insert(id, new_node);
        assert!(
            old_node.is_none(),
            "can't add a node that is already present"
        );
    }

    pub fn build_pos_to_node_id(&self) -> HashMap<Point, SupportNodeId> {
        self.nodes.iter().map(|x| (x.1.position, *x.0)).collect()
    }

    #[allow(dead_code)]
    pub fn add_link(&mut self, from_id: SupportNodeId, to_id: SupportNodeId) {
        let from_position = self
            .nodes
            .get(&from_id)
            .expect("node shall be found")
            .position;
        let to = self.nodes.get_mut(&to_id).expect("node shall be found");

        let distance = (from_position - to.position).abs();

        to.neighbors.push(Neighbor {
            id: from_id,
            distance,
        });

        self.nodes
            .get_mut(&from_id)
            .expect("node shall be found")
            .neighbors
            .push(Neighbor {
                id: to_id,
                distance,
            });
    }
}
