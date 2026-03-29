use baby_shark::mesh::corner_table::FaceId;
use hashbrown::HashMap;
use smallvec::SmallVec;

use crate::models::Point;



pub struct SupportNodeId(u32);

pub struct SupportNodeGene {
    id: SupportNodeId,
    connections: SmallVec<[SupportConnection;4]>
}


#[allow(clippy::enum_variant_names)]
pub enum SupportConnection{
    /// connect to a surface of the printed structure
    /// (this will have a cost in terms of cleanliness
    /// ot the final print, but could reduce the material used)
    ToMesh(FaceId),

    /// link the graph to a node that should already exist.
    /// If the node does not exist, then the link will be ignored.
    ToExistingNode(SupportNodeId),

    /// create a new node. The node will have a position
    /// that is offset-ed a certain amount from the position of
    /// the current node
    ToNewNode{new_node_id: SupportNodeId, offset: Point}
}

pub struct SupportStructureGenome {
    /// fixed nodes (can be mutated, but not removed)
    /// represent the connections to the contact points
    pub fixed_nodes: HashMap<SupportNodeId, SupportNodeGene>,
    /// dynamics nodes (can be mutated and removed)
    /// represent nodes in the middle that are used for supporting
    /// the nodes in the `fixed_nodes` set.
    pub dynamics_nodes: HashMap<SupportNodeId, SupportNodeGene>,
}


pub enum SupportNodeKind {
    /// base point (is attached to the ground or to a stable
    /// part of the mesh)
    Base,
    /// middle point (is neither a contact nor a based point)
    Middle,
    /// contact point (makes contact with the mesh and provide support)
    Contact
}

pub struct SupportNode {
    pub kind: SupportNodeKind,
    pub position: Point,
    pub id: SupportNodeId,
    pub neighbors: SmallVec<[SupportNodeId;4]>

}

pub struct SupportGraph {
    pub nodes: HashMap<SupportNodeId, SupportNode>
}
