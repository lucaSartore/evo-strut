use hashbrown::{HashMap, hash_set::Iter};
use itertools::Itertools;
use nalgebra::Matrix2;
use rerun::demo_util::grid;
use smallvec::smallvec;

use crate::{evolution::{Cost, Random}, models::{Point, SurfaceGraph}, stages::support_structure_refinement::{BaseNode, ContactNode, MiddleNode, PositionAnchor, SupportNode, SupportNodeId, SupportStructureGene}};


#[derive(Clone, Debug)]
pub struct CompressedSupportGene {
    pub groups: Vec<SupportGroup>
}

#[derive(Clone, Debug)]
pub struct ContactPoint {
    pub position: Point,
    pub radius: f32,
}

#[derive(Clone, Debug)]
pub struct SupportLayer {
    pub center: Point,
    pub nodes: Vec<LayerNode>
}

impl SupportLayer {
    /// return the position of the nodes from this layer that are
    /// the closest to a certain point
    pub fn closest_points_to_point(&self, point: Point) -> Vec<Point> {
        let mut v: Vec<Point> = self.nodes_positions().collect();
        v.sort_by_key(|x| Cost::new((*x - point).abs()));
        v
    }

    pub fn closest_point_to_point(&self, point: Point) -> Point {
        self.nodes_positions()
            .min_by_key(|x| Cost::new((*x - point).abs()))
            .expect("a layer can't have zero nodes")
    }

    pub fn nodes_positions(&self) -> impl Iterator<Item = Point> {
        self
            .nodes
            .iter()
            .map(|x| self.center + x.offset)
    }
}

#[derive(Clone, Debug)]
pub struct LayerNode {
    /// position offset w.r.t. the center of the layer.
    /// the z index should always be zero
    pub offset: Point,
    pub connections: LayerConnections
}

#[derive(Clone, Debug)]
pub struct LayerConnections {
    // pub connect_to_closest_surface: bool,
    pub connect_to_closest_below_layer_node: bool,
    pub connect_to_second_closest_below_layer_node: bool,
    pub connect_to_third_closest_below_layer_node: bool
}

#[derive(Clone, Debug)]
pub struct SupportGroup {
    pub supports: Vec<ContactPoint>,
    pub layers: Vec<SupportLayer>
}
impl SupportGroup {
    pub fn max_height(&self) -> f32 {
        self.support_positions()
            .max_by_key(|x| Cost::new(x.z))
            .expect("each group must contains at least one support")
            .z
    }

    // calculate the mean and covariance matrix of the distribution of supports
    // at a certain height.
    // is based on al the nodes that are above the height (including
    // both support nodes, and nodes that are part of layers
    pub fn mean_and_cov(&self, height: f32) -> (Point, Matrix2<f32>) {
        let points: Vec<Point> = self
            .layers
            .iter()
            .flat_map(|x| x.nodes_positions())
            .chain(self.support_positions())
            .filter(|x| x.z > height)
            .map(|mut x| {x.z = 0.; x})
            .collect();
        let n = points.len() as f32;

        assert_ne!(n, 0., "there shall always be at least a point above the height");

        let sum_point = points.iter().fold(Point::ZERO, |acc, p| acc + *p);
        let mean = sum_point.to_scaled(1./n);

        let mut cov_xx = 0.0;
        let mut cov_yy = 0.0;
        let mut cov_xy = 0.0;

        for p in &points {
            let dx = p.x - mean.x;
            let dy = p.y - mean.y;
            
            cov_xx += dx * dx;
            cov_yy += dy * dy;
            cov_xy += dx * dy;
        }

        let covariance_matrix = Matrix2::new(
            cov_xx / n, cov_xy / n,
            cov_xy / n, cov_yy / n,
        );

        (mean, covariance_matrix)
    }

    pub fn support_positions(&self) -> impl Iterator<Item = Point> {
        self.supports
            .iter()
            .map(|x| x.position)
    }
}

impl CompressedSupportGene {
    pub fn to_full_genes(&self, graph: & SurfaceGraph) -> Vec<SupportStructureGene> {
        let mut to_return = Vec::with_capacity(self.groups.len());
        for g in &self.groups {
            let mut builder = RawStructureBuilder::new();
            builder.add_group(g);
            to_return.push(builder.build(graph));
        }
        to_return
    }
    pub fn to_full_gene(&self, graph: & SurfaceGraph) -> SupportStructureGene {
        let mut builder = RawStructureBuilder::new();
        for g in &self.groups {
            builder.add_group(g);
        }
        builder.build(graph)
    }

    pub fn rand_group_mut(&mut self, rand: &Random) -> &mut SupportGroup {
        rand.choose_mut(&mut self.groups)
            .expect("there shall always be at least a group")
    }
    pub fn rand_group(&self, rand: &Random) -> &SupportGroup {
        rand.choose(&self.groups)
            .expect("there shall always be at least a group")
    }
}

struct RawStructureBuilder {
    pub raw_structure: SupportStructureGene,
    pub position_to_id: HashMap<Point, SupportNodeId>,
    pub random: Random
}

impl RawStructureBuilder {
    pub fn new() -> Self {
        // node zero is added as a placeholder, so that the we can use it for anchors, and then
        // remove it and repairing the structure
        let mut raw_structure = SupportStructureGene { nodes: Default::default() };
        raw_structure.nodes.insert(SupportNodeId(0), SupportNode::Base(BaseNode { id: SupportNodeId(0), mesh_contact: None, last_position: Point::ZERO }));
        Self {
            raw_structure,
            position_to_id: Default::default(),
            // random has no actual effect on the shape of the created structure
            // it only effect IDs, so we can put an unseeded value here
            random: Random::UnSeededRandom
        }
    }

    pub fn build(mut self, graph: & SurfaceGraph) -> SupportStructureGene {
        self.raw_structure.nodes.remove(&SupportNodeId(0));
        // the structure passed will already be completed, so the random will
        // have no effects. The repair will only fix-up the contacts
        // to the ground and the anchors
        self.raw_structure.repair(graph, &self.random);
        self.raw_structure
    }


    pub fn add_group(&mut self, group: &SupportGroup) {
        enum ListElement<'a> {
            Layer(&'a SupportLayer),
            Point(&'a ContactPoint)
        }
        impl<'a> ListElement<'a> {
            pub fn height(&self) -> f32 {
                match self {
                    ListElement::Layer(support_layer) => support_layer.center.z,
                    ListElement::Point(contact_point) => contact_point.position.z
                }
            }
        }

        let mut vec = vec![];
        for layer in &group.layers {
            vec.push(ListElement::Layer(layer));
        }
        for support in &group.supports {
            vec.push(ListElement::Point(support));
        }
        vec.sort_by_key(|x| Cost::new(x.height()));

        let mut prev_layer = None;
        for e in vec {
            match e {
                ListElement::Layer(support_layer) => {
                    self.add_layer(support_layer, prev_layer);
                    prev_layer = Some(support_layer);
                }
                ListElement::Point(contact_point) => {
                    self.add_contact(contact_point, prev_layer);
                }
            }
        }
    }

    pub fn create_node(&mut self, position: Point, node_kind: NodeKind) -> SupportNodeId {
        if let Some(k) = self.position_to_id.get(&position) {
            return *k
        }
        let id = self.raw_structure.new_random_id(&self.random);
        let n = match node_kind {
            NodeKind::Contact { radius } => {
                let n = ContactNode{
                    id,
                    position,
                    radius,
                    leans_on: smallvec![]
                };
                SupportNode::Contact(n)
            },
            NodeKind::Middle => {
                let n = MiddleNode{
                    id,
                    // we put a random anchor
                    anchor: PositionAnchor::new(SupportNodeId(0), Point::ZERO, Point::ZERO),
                    last_position: position,
                    leans_on: smallvec![]
                };
                SupportNode::Middle(n)
            }
        };

        self.raw_structure.nodes.insert(id, n);
        self.position_to_id.insert(position, id);
        id
    }

    pub fn add_connection(&mut self, from: Point, to: Point) {
        let from_id = self.position_to_id[&from];
        let to_id = self.position_to_id[&to];
        self.raw_structure
            .nodes
            .get_mut(&from_id)
            .expect("from node shall always be found")
            .add_leans_on(to_id);
    }

    pub fn add_layer(&mut self, layer: &SupportLayer, layer_below: Option<&SupportLayer>) {
        layer.nodes_positions()
            .for_each(|x| { 
                let _ = self.create_node(x, NodeKind::Middle); 
            });
        let Some(layer_below) = layer_below else { return };
        layer.nodes
            .iter()
            .for_each(|x| { 
                let x_position = layer.center + x.offset;
                let closest = layer_below.closest_points_to_point(x_position);
                if x.connections.connect_to_closest_below_layer_node && let Some(below) = closest.first() {
                    self.add_connection(x_position, *below);
                }
                if x.connections.connect_to_second_closest_below_layer_node && let Some(below) = closest.get(1) {
                    self.add_connection(x_position, *below);
                }
                if x.connections.connect_to_third_closest_below_layer_node && let Some(below) = closest.get(2) {
                    self.add_connection(x_position, *below);
                }
            });
    }

    pub fn add_contact(&mut self, contact: &ContactPoint, layer_below: Option<&SupportLayer>) {
        let _ = self.create_node(contact.position, NodeKind::Contact { radius: contact.radius });
        let Some(layer_below) = layer_below else { return };
        let support = layer_below.closest_point_to_point(contact.position);
        self.add_connection(contact.position, support);
    }
}


enum NodeKind {
    Contact{radius: f32},
    Middle
}
