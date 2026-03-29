use std::cell::RefCell;

use hashbrown::{HashMap, HashSet};
use smallvec::SmallVec;

use crate::{evolution::Random, models::{FaceId, Point, SurfaceGraph}};



#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct SupportNodeId(u32);


#[derive(Clone, Debug)]
struct PositionAnchor {
    pub to: SupportNodeId,
    pub offset: Point
}

/// represent a base point (i.e. a point that is connected either to the ground,
/// or to the printed mesh itself, and provide "support to the support structure")
#[derive(Clone, Debug)]
pub struct BaseNode {
    pub id: SupportNodeId,
    // select the face that the current node leans on.
    // if None, then the current node leans to the ground.
    pub mesh_contact: Option<FaceId>,
    pub last_position: Point
}

impl BaseNode {
    pub fn new_ground(id: SupportNodeId, position: Point) -> Self {
        Self {
            id,
            mesh_contact: None,
            last_position: position
        }
    }
    pub fn new_mesh_contact(id: SupportNodeId, contact: FaceId, graph: &SurfaceGraph) -> Self {
        Self {
            id,
            mesh_contact: Some(contact),
            last_position: graph.get_triangle(contact).center()
        }
    }
    pub fn repair_position(&mut self, prev_point: &NodeReference, graph: &SurfaceGraph) {
        if let Some(e) = self.mesh_contact {
            self.last_position = graph.get_triangle(e).center();
        } else {
            self.last_position = prev_point.position;
            self.last_position.z = 0.;
        }
    }
}

/// represent a middle node (i.e. a structural node that is in between
/// base and contact nodes
#[derive(Clone, Debug)]
pub struct MiddleNode {
    pub id: SupportNodeId,
    // this node's position will be anchor.to.position + anchor.offset
    pub anchor: PositionAnchor,
    // kept to re-construct the position in case node we ar anchoring to
    // is deleted
    pub last_position: Point, 
    pub leans_on: SmallVec<[SupportNodeId; 4]>
}

#[derive(Clone, Debug)]
pub struct NodeReference {
    id: SupportNodeId,
    position: Point
}


impl MiddleNode {
    // repair the anchor by building a new one if the node i'm anchored to has being
    // deleted (or no longer depends on me)
    pub fn repair_position(&mut self, genome: &SupportStructureGenome, prev_point: &NodeReference) {
        // no need to repair
        if let Some(g) = genome.try_get_gene(prev_point.id) && g.leans_on(self.id) {
            self.last_position = g.get_position() + self.anchor.offset;
        }
        // repairing the anchor with the new node
        self.anchor.to = prev_point.id;
        self.anchor.offset = self.last_position - prev_point.position;
    }

    pub fn as_node_reference(&self) -> NodeReference {
        NodeReference { 
            id: self.id,
            position: self.last_position
        }
    }
}

/// represent a contact point (i.e. a point that is providing
/// support to the mesh we are printing)
#[derive(Clone, Debug)]
pub struct ContactNode {
    pub id: SupportNodeId,
    pub position: Point,
    pub radius: f32,
    pub leans_on: SmallVec<[SupportNodeId; 4]>
}

impl ContactNode {
    pub fn as_node_reference(&self) -> NodeReference {
        NodeReference { 
            id: self.id,
            position: self.position
        }
    }

}

#[derive(Clone, Debug)]
pub enum SupportNodeGene {
    BaseNode(RefCell<BaseNode>),
    MiddleNode(RefCell<MiddleNode>),
    ContactNode(RefCell<ContactNode>)
}

impl SupportNodeGene {
    pub fn leans_on(&self, id: SupportNodeId) -> bool {
        match self {
            SupportNodeGene::BaseNode(_) => false,
            SupportNodeGene::MiddleNode(n) => n.borrow().leans_on.contains(&id),
            SupportNodeGene::ContactNode(n) => n.borrow().leans_on.contains(&id)
        }
    }

    pub fn get_position(&self) -> Point {
        match self {
            SupportNodeGene::BaseNode(n) => n.borrow().last_position,
            SupportNodeGene::MiddleNode(n) => n.borrow().last_position,
            SupportNodeGene::ContactNode(n) => n.borrow().position
        }
    }

    pub fn is_floating(&self) -> bool {
        match self {
            SupportNodeGene::BaseNode(_) => false,
            SupportNodeGene::MiddleNode(n) => n.borrow().leans_on.is_empty(),
            SupportNodeGene::ContactNode(n) => n.borrow().leans_on.is_empty(),
        }
    }

    pub fn add_support(&self, support: SupportNodeId) {
        match self {
            SupportNodeGene::BaseNode(_) => panic!("can't add support on base node"),
            SupportNodeGene::MiddleNode(n) => n.borrow_mut().leans_on.push(support),
            SupportNodeGene::ContactNode(n) => n.borrow_mut().leans_on.push(support),
        };
    }

}

#[derive(Debug)]
pub struct SupportStructureGenome {
    pub nodes: HashMap<SupportNodeId, SupportNodeGene>,
}

impl SupportStructureGenome {
    pub fn new_random_id(&self, rand: &Random) -> SupportNodeId {
        let id = SupportNodeId(rand.next_u32());
        // re-generate it, as it is already taken
        if self.is_id_present(id) {
            return self.new_random_id(rand);
        }
        id
    }

    pub fn is_id_present(&self, id: SupportNodeId) -> bool {
        self.nodes.contains_key(&id)
    }

    pub fn get_gene(&self, id: SupportNodeId) -> &SupportNodeGene {
        &self.nodes[&id]
    }

    pub fn try_get_gene(&self, id: SupportNodeId) -> Option<&SupportNodeGene> {
        self.nodes.get(&id)
    }

    pub fn has_gene(&self, id: SupportNodeId) -> bool {
        self.nodes.contains_key(&id)
    }

    pub fn repair(&mut self, graph: &SurfaceGraph, rand: &Random) {
        let mut repaired = Default::default();
        // repair all the nodes
        for n in self.nodes.values() {
            if let SupportNodeGene::ContactNode(n) = n {
                self.repair_node_position(
                    n.borrow().id,
                    None,
                    &mut repaired,
                    graph
                );
            }
        }
        // remove nodes that were not repaired
        self.nodes.retain(|x,_| repaired.contains(x));

        // fix the nodes that are floating
        self.repair_floating_nodes(rand);
    }

    fn repair_floating_nodes(&mut self, rand: &Random) {
        let floating: Vec<_> = self
            .nodes
            .iter()
            .filter(|x| x.1.is_floating())
            .map(|x| x.0)
            .copied()
            .collect();

        for f in &floating {
            let support = self.new_random_id(rand);
            let node = self.nodes.get_mut(f).expect("node should always exit");
            node.add_support(support);
            let mut position = node.get_position();
            position.z = 0.;
            self.nodes.insert(support, SupportNodeGene::BaseNode(
                BaseNode::new_ground(support, position).into()
            ));
        }
    }

    // try to repair the node. Return true if the repair succeeded, false otherwise.
    fn repair_node_position(&self, id: SupportNodeId, prev_point: Option<&NodeReference>, repaired_nodes: &mut HashSet<SupportNodeId>, graph: &SurfaceGraph) -> bool {
        match self.nodes.get(&id) {
            None => {
                // node is not present... can't be repaired
                return false;
            }
            Some(SupportNodeGene::BaseNode(n)) => {
                let pp = prev_point.expect("only contact nodes can have prev_point = none");
                let mut n_mut = n.borrow_mut();

                // repair just the position (has no dependencies)
                n_mut.repair_position(pp, graph);
                repaired_nodes.insert(n_mut.id);
                return true;
            },
            Some(SupportNodeGene::ContactNode(n)) => {
                let mut n_mut = n.borrow_mut();

                // keep only the nodes successfully repaired
                let this_point = n_mut.as_node_reference();
                n_mut.leans_on.retain(|x| 
                    self.repair_node_position(*x, Some(&this_point), repaired_nodes, graph)
                );

                repaired_nodes.insert(n_mut.id);
                return true;
            },
            Some(SupportNodeGene::MiddleNode(n)) => {
                let pp = prev_point.expect("only contact nodes can have prev_point = none");
                let mut n_mut = n.borrow_mut();

                // repairing the position
                n_mut.repair_position(self, pp);

                // keep only the nodes successfully repaired
                let this_point = n_mut.as_node_reference();
                n_mut.leans_on.retain(
                    |x| self.repair_node_position(*x, Some(&this_point), repaired_nodes, graph)
                );

                repaired_nodes.insert(n_mut.id);
                return true;
            }
        };
    }
}
