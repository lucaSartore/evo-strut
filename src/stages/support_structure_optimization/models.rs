use std::cell::RefCell;

use hashbrown::{HashMap, HashSet};
use smallvec::SmallVec;

use crate::{evolution::Random, models::{FaceId, Point, SurfaceGraph, SurfaceNode}};



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
    pub fn repair_position(&self, prev_point: &NodeReference, graph: &SurfaceGraph) -> Self {
        let mut to_return = self.clone();
        if let Some(e) = self.mesh_contact {
            to_return.last_position = graph.get_triangle(e).center();
        } else {
            to_return.last_position = prev_point.position;
            to_return.last_position.z = 0.;
        }
        to_return
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
    pub fn repair_position(&self, genome: &SupportStructureGene, prev_point: &NodeReference) -> Self {
        let mut to_return = self.clone();
        // anchor still present
        if let Some(g) = genome.try_get_gene(self.anchor.to) && g.leans_on(self.id) {
            to_return.last_position = g.get_position() + self.anchor.offset;
            return to_return
        }
        // repairing the anchor with a new node
        to_return.anchor.to = prev_point.id;
        to_return.anchor.offset = self.last_position - prev_point.position;
        to_return
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
pub enum SupportNode {
    BaseNode(BaseNode),
    MiddleNode(MiddleNode),
    ContactNode(ContactNode)
}

impl SupportNode {
    pub fn leans_on(&self, id: SupportNodeId) -> bool {
        match self {
            SupportNode::BaseNode(_) => false,
            SupportNode::MiddleNode(n) => n.leans_on.contains(&id),
            SupportNode::ContactNode(n) => n.leans_on.contains(&id)
        }
    }

    pub fn get_position(&self) -> Point {
        match self {
            SupportNode::BaseNode(n) => n.last_position,
            SupportNode::MiddleNode(n) => n.last_position,
            SupportNode::ContactNode(n) => n.position
        }
    }

    pub fn is_floating(&self) -> bool {
        match self {
            SupportNode::BaseNode(_) => false,
            SupportNode::MiddleNode(n) => n.leans_on.is_empty(),
            SupportNode::ContactNode(n) => n.leans_on.is_empty(),
        }
    }

    pub fn add_support(&mut self, support: SupportNodeId) {
        match self {
            SupportNode::BaseNode(_) => panic!("can't add support on base node"),
            SupportNode::MiddleNode(n) => n.leans_on.push(support),
            SupportNode::ContactNode(n) => n.leans_on.push(support),
        };
    }

}

#[derive(Debug, Clone)]
pub struct SupportStructureGene {
    pub nodes: HashMap<SupportNodeId, SupportNode>,
}

impl SupportStructureGene {
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

    pub fn get_gene(&self, id: SupportNodeId) -> &SupportNode {
        &self.nodes[&id]
    }

    pub fn try_get_gene(&self, id: SupportNodeId) -> Option<&SupportNode> {
        self.nodes.get(&id)
    }

    pub fn has_gene(&self, id: SupportNodeId) -> bool {
        self.nodes.contains_key(&id)
    }

    pub fn repair(&mut self, graph: &SurfaceGraph, rand: &Random) {
        let mut repaired = Default::default();
            
        let ids: Vec<SupportNodeId> = self
            .nodes
            .values()
            .filter_map(|x| {
                match x {
                    SupportNode::ContactNode(n) => Some(n.id),
                    _ => None,
                }
            })
            .collect();

        // repair all the nodes
        for id in ids {
            self.repair_node_position(
                id,
                None,
                &mut repaired,
                graph
            );
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
            self.nodes.insert(support, SupportNode::BaseNode(
                BaseNode::new_ground(support, position).into()
            ));
        }
    }

    // try to repair the node. Return true if the repair succeeded, false otherwise.
    fn repair_node_position(&mut self, id: SupportNodeId, prev_point: Option<&NodeReference>, repaired_nodes: &mut HashSet<SupportNodeId>, graph: &SurfaceGraph) -> bool {
        match self.nodes.get(&id) {
            None => {
                // node is not present... can't be repaired
                return false
            }
            Some(SupportNode::BaseNode(n)) => {
                let pp = prev_point.expect("only contact nodes can have prev_point = none");
                let repaired = n.repair_position(pp, graph);
                self.nodes.insert(id, SupportNode::BaseNode(repaired));
                repaired_nodes.insert(id);
            },
            Some(SupportNode::ContactNode(n)) => {
                let this_point = n.as_node_reference();
                let mut lean_on = n.leans_on.clone();

                lean_on.retain(|x|
                    self.repair_node_position(*x, Some(&this_point), repaired_nodes, graph)
                );

                let Some(SupportNode::ContactNode(n)) = self.nodes.get_mut(&id) else { panic!() };
                n.leans_on = lean_on;
                repaired_nodes.insert(id);
            },
            Some(SupportNode::MiddleNode(n)) => {
                let this_point = n.as_node_reference();
                let pp = prev_point.expect("only contact nodes can have prev_point = none");

                // repairing current 
                let mut repaired = n.repair_position(self, pp);

                // update the last position of self, before progressing on the downward nodes
                let Some(SupportNode::MiddleNode(n)) = self.nodes.get_mut(&id) else { panic!() };
                n.last_position = repaired.last_position;

                repaired.leans_on.retain(|x|
                    self.repair_node_position(*x, Some(&this_point), repaired_nodes, graph)
                );

                self.nodes.insert(id, SupportNode::MiddleNode(repaired));
            }
        };
        true
    }
}
