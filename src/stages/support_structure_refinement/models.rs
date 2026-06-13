use crate::support::remove_random::RemoveRandom;
use hashbrown::{HashMap, HashSet};
use smallvec::SmallVec;

use crate::{
    evolution::Random,
    models::{FaceId, Point, SurfaceGraph},
};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct SupportNodeId(pub u32);

#[derive(Clone, Debug)]
pub struct PositionAnchor {
    pub to: SupportNodeId,
    pub offset: Point,
}

impl PositionAnchor {
    pub fn new(
        node_to: SupportNodeId,
        node_to_position: Point,
        anchored_node_position: Point,
    ) -> Self {
        Self {
            to: node_to,
            offset: anchored_node_position - node_to_position,
        }
    }
}

/// represent a base point (i.e. a point that is connected either to the ground,
/// or to the printed mesh itself, and provide "support to the support structure")
#[derive(Clone, Debug)]
pub struct BaseNode {
    pub id: SupportNodeId,
    // select the face that the current node leans on.
    // if None, then the current node leans to the ground.
    pub mesh_contact: Option<FaceId>,
    pub last_position: Point,
    pub radius: f32
}

impl BaseNode {
    pub fn new_ground(id: SupportNodeId, position: Point, radius: f32) -> Self {
        Self {
            id,
            mesh_contact: None,
            last_position: position,
            radius
        }
    }
    pub fn new_mesh_contact(id: SupportNodeId, contact: FaceId, graph: &SurfaceGraph) -> Self {
        Self {
            id,
            mesh_contact: Some(contact),
            last_position: graph.get_triangle(contact).center(),
            radius: 3.0
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
    pub radius: f32,
    pub leans_on: SmallVec<[SupportNodeId; 4]>,
}

#[derive(Clone, Debug)]
pub struct NodeReference {
    id: SupportNodeId,
    position: Point,
}

impl MiddleNode {
    // repair the anchor by building a new one if the node i'm anchored to has being
    // deleted (or no longer depends on me)
    pub fn repair_position(
        &self,
        genome: &SupportStructureGene,
        prev_point: &NodeReference,
    ) -> Self {
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
            position: self.last_position,
        }
    }
    pub fn remove_random_support(&mut self, rand: &Random) {
        if !self.leans_on.is_empty() {
            let i = rand.next_in_range(0, self.leans_on.len() as u64);
            self.leans_on.remove(i as usize);
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
    pub leans_on: SmallVec<[SupportNodeId; 4]>,
}

impl ContactNode {
    pub fn as_node_reference(&self) -> NodeReference {
        NodeReference {
            id: self.id,
            position: self.position,
        }
    }

    pub fn remove_random_support(&mut self, rand: &Random) {
        if !self.leans_on.is_empty() {
            let i = rand.next_in_range(0, self.leans_on.len() as u64);
            self.leans_on.remove(i as usize);
        }
    }
}

#[derive(Clone, Debug)]
pub enum SupportNode {
    Base(BaseNode),
    Middle(MiddleNode),
    Contact(ContactNode),
}

impl SupportNode {
    pub fn leans_on(&self, id: SupportNodeId) -> bool {
        match self {
            SupportNode::Base(_) => false,
            SupportNode::Middle(n) => n.leans_on.contains(&id),
            SupportNode::Contact(n) => n.leans_on.contains(&id),
        }
    }

    pub fn add_leans_on(&mut self, id: SupportNodeId) {
        match self {
            SupportNode::Base(_) => (),
            SupportNode::Middle(n) => n.leans_on.push(id),
            SupportNode::Contact(n) => n.leans_on.push(id),
        };
    }

    pub fn id(&self) -> SupportNodeId {
        match self {
            SupportNode::Base(n) => n.id,
            SupportNode::Middle(n) => n.id,
            SupportNode::Contact(n) => n.id,
        }
    }

    pub fn radius(&self) -> f32 {
        match self {
            SupportNode::Base(n) => n.radius,
            SupportNode::Middle(n) => n.radius,
            SupportNode::Contact(n) => n.radius,
        }
    }

    pub fn get_position(&self) -> Point {
        match self {
            SupportNode::Base(n) => n.last_position,
            SupportNode::Middle(n) => n.last_position,
            SupportNode::Contact(n) => n.position,
        }
    }

    pub fn is_floating(&self) -> bool {
        match self {
            SupportNode::Base(_) => false,
            SupportNode::Middle(n) => n.leans_on.is_empty(),
            SupportNode::Contact(n) => n.leans_on.is_empty(),
        }
    }

    pub fn add_support(&mut self, support: SupportNodeId) {
        match self {
            SupportNode::Base(_) => panic!("can't add support on base node"),
            SupportNode::Middle(n) => n.leans_on.push(support),
            SupportNode::Contact(n) => n.leans_on.push(support),
        };
    }

    pub fn is_base(&self) -> bool {
        matches!(self, SupportNode::Base(_))
    }
    pub fn is_middle(&self) -> bool {
        matches!(self, SupportNode::Middle(_))
    }
    pub fn is_contact(&self) -> bool {
        matches!(self, SupportNode::Contact(_))
    }

    pub fn remove_random_support(&mut self, rand: &Random) {
        match self {
            SupportNode::Base(_) => (),
            SupportNode::Middle(n) => n.remove_random_support(rand),
            SupportNode::Contact(n) => n.remove_random_support(rand),
        }
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

    pub fn is_supported(&self, id: SupportNodeId) -> bool {
        self.nodes[&id].is_base()
    }

    pub fn is_contact(&self, id: SupportNodeId) -> bool {
        self.nodes[&id].is_contact()
    }

    pub fn random_non_base_node(&mut self, rand: &Random) -> SupportNodeId {
        loop {
            let v = self
                .nodes
                .choose_random(rand)
                .expect("can't execute random selection on an empty graph");
            if !v.1.is_base() {
                return v.0;
            }
        }
    }

    pub fn random_non_contact_node(&mut self, rand: &Random) -> SupportNodeId {
        loop {
            let v = self
                .nodes
                .choose_random(rand)
                .expect("can't execute random selection on an empty graph");
            if !v.1.is_contact() {
                return v.0;
            }
        }
    }

    pub fn random_contact_node(&mut self, rand: &Random) -> SupportNodeId {
        loop {
            let v = self
                .nodes
                .choose_random(rand)
                .expect("can't execute random selection on an empty graph");
            if v.1.is_contact() {
                return v.0;
            }
        }
    }

    pub fn random_middle_node(&mut self, rand: &Random) -> Option<SupportNodeId> {
        for _ in 0..5 {
            let v = self
                .nodes
                .choose_random(rand)
                .expect("can't execute random selection on an empty graph");
            if v.1.is_middle() {
                return Some(v.0);
            }
        }
        None
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

        // todo: there is a bug in the function, and the entire thing could probably be simplified
        self.repair_floating_nodes(rand);
        return;
        let mut repaired = Default::default();

        let ids: Vec<SupportNodeId> = self
            .nodes
            .values()
            .filter_map(|x| match x {
                SupportNode::Contact(n) => Some(n.id),
                _ => None,
            })
            .collect();

        // repair all the nodes
        for id in ids {
            let mut visited = Default::default();
            self.repair_node_position(id, None, &mut repaired, &mut visited, graph);
        }
        // remove nodes that were not repaired
        self.nodes.retain(|x, _| repaired.contains(x));

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
            let radius = node.radius();
            position.z = 0.;
            self.nodes.insert(
                support,
                SupportNode::Base(BaseNode::new_ground(support, position, radius)),
            );
        }
    }

    // try to repair the node. Return true if the repair succeeded, false otherwise.
    fn repair_node_position(
        &mut self,
        id: SupportNodeId,
        prev_point: Option<&NodeReference>,
        repaired_nodes: &mut HashSet<SupportNodeId>,
        visited: &mut HashSet<SupportNodeId>,
        graph: &SurfaceGraph,
    ) -> bool {
        // we return false if the node has already being visited in this repair
        // loop. This is done to avoid creating circular support structure
        if visited.contains(&id) {
            return false;
        }
        visited.insert(id);
        match self.nodes.get(&id) {
            None => {
                // node is not present... can't be repaired
                return false;
            }
            Some(SupportNode::Base(n)) => {
                let pp = prev_point.expect("only contact nodes can have prev_point = none");
                let repaired = n.repair_position(pp, graph);
                self.nodes.insert(id, SupportNode::Base(repaired));
                repaired_nodes.insert(id);
            }
            Some(SupportNode::Contact(n)) => {
                let this_point = n.as_node_reference();
                let mut lean_on = n.leans_on.clone();

                lean_on.retain(|x| {
                    self.repair_node_position(*x, Some(&this_point), repaired_nodes, visited, graph)
                });

                let Some(SupportNode::Contact(n)) = self.nodes.get_mut(&id) else { panic!() };
                n.leans_on = lean_on;
                repaired_nodes.insert(id);
            }
            Some(SupportNode::Middle(n)) => {
                let this_point = n.as_node_reference();
                let pp = prev_point.expect("only contact nodes can have prev_point = none");

                // repairing current
                let mut repaired = n.repair_position(self, pp);

                // update the last position of self, before progressing on the downward nodes
                let Some(SupportNode::Middle(n)) = self.nodes.get_mut(&id) else { panic!() };
                n.last_position = repaired.last_position;

                repaired.leans_on.retain(|x| {
                    self.repair_node_position(*x, Some(&this_point), repaired_nodes, visited, graph)
                });

                self.nodes.insert(id, SupportNode::Middle(repaired));
                repaired_nodes.insert(id);
            }
        };
        true
    }
}
