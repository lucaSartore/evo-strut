use core::f32;
use std::fmt::Debug;

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use serde::Serialize;

use crate::{
    evolution::{Cost, Random},
    models::{FaceId, Point, Settings, SurfaceGraph},
    stages::support_structure_refinement::{
        BaseNode, ContactNode, MiddleNode, PositionAnchor, SupportNode, SupportNodeId,
        SupportStructureGene,
    },
    support::{
        convex_hull::ConvexHull,
        neural_network::{
            ActivationFunction, LayerTopology, NetworkTopology, NetworkWeightInitialization,
            NeuralNetwork,
        },
    },
};

#[derive(Clone, Debug, Copy, Serialize)]
pub struct ContactPoint {
    pub face: FaceId,
    pub position: Point,
    pub radius: f32,
}

#[derive(Clone, Debug, Copy, Serialize)]
pub struct SupportPoint {
    pub position: Point,
    pub num_contacts: u32,
}

impl SupportPoint {
    pub fn radius(&self, full_gene: &SupportStructureOptimizationGene) -> f32 {
        // let p: [f32; 3] = self.position.into();
        // let output = full_gene.contact_radius.evaluate(&p).expect("network evaluation failed")[0];
        // todo: hardcoded values
        // println!("output: {output}");
        // return output * (5. - 1.5) + 0.5;
        1.5
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct SupportStructureOptimizationGene {
    pub contacts: Vec<ContactPoint>,
    pub supports: Vec<SupportPoint>,
    pub contact_radius: NeuralNetwork,
    pub convex_hull: ConvexHull,
}

impl SupportStructureOptimizationGene {
    pub fn from_contacts(contacts: Vec<ContactPoint>, random: &Random) -> Self {
        let convex_hull = ConvexHull::new(contacts.iter().map(|x| x.position).collect());
        Self {
            contacts,
            supports: vec![],
            contact_radius: Self::random_network(random),
            convex_hull,
        }
    }

    pub fn random_network(random: &Random) -> NeuralNetwork {
        let topology = NetworkTopology::new(
            3,
            vec![
                // LayerTopology::new(16, ActivationFunction::Relu)
                //     .expect("invalid default contact-point grouping hidden layer"),
                LayerTopology::new(1, ActivationFunction::Sigmoid)
                    .expect("invalid default contact-point grouping output layer"),
            ],
        )
        .unwrap();
        let initialization = NetworkWeightInitialization::He;
        NeuralNetwork::random(topology, initialization, random).unwrap()
    }

    pub fn to_full_gene(&self, graph: &SurfaceGraph, settings: &Settings) -> SupportStructureGene {
        let builder = RawStructureBuilder::new(settings, self);
        builder.build(graph)
    }

    pub fn random_point(&self, rand: &Random) -> Point {
        let n_supports = self.supports.len();
        let n_contact = self.contacts.len();
        let pick_from_support_prob = n_supports as f32 / (n_supports + n_contact) as f32;
        if rand.random_choice(pick_from_support_prob) {
            rand.choose(&self.supports).unwrap().position
        } else {
            rand.choose(&self.contacts).unwrap().position
        }
    }

    pub fn random_point_close_to(
        &self,
        original_position: Point,
        alpha: f32,
        min_angle_rad: f32,
        rand: &Random,
    ) -> Point {
        // probabilities that are too small risk to make this function slow
        assert!(alpha > 0.001);
        let options: Vec<_> = self
            .all_points_positions()
            .filter(|x| {
                *x != original_position
                    && Point::horizon_angle(*x, original_position) > min_angle_rad
            })
            .map(|x| (x, (x - original_position).norm_sq()))
            .sorted_by_key(|x| Cost::new(x.1))
            .map(|x| x.0)
            .collect();

        if options.is_empty() {
            return original_position;
        }

        let mut index = 0;

        loop {
            if rand.random_choice(alpha) {
                return options[index];
            }
            index = (index + 1) % options.len();
        }
    }

    pub fn random_support_mut(&mut self, rand: &Random) -> Option<&mut SupportPoint> {
        rand.choose_mut(&mut self.supports)
    }

    pub fn contact_positions(&self) -> Vec<Point> {
        self.contacts.iter().map(|x| x.position).collect()
    }

    pub fn all_points_positions(&self) -> impl Iterator<Item = Point> {
        self.contacts
            .iter()
            .map(|x| x.position)
            .chain(self.supports.iter().map(|x| x.position))
    }

    pub fn max_height(&self) -> f32 {
        self.contacts
            .iter()
            .map(|x| Cost::new(x.position.z))
            .max()
            .expect("there shall always be a support")
            .as_f32()
    }
}

struct RawStructureBuilder<'a> {
    pub optimization_structure: &'a SupportStructureOptimizationGene,
    pub raw_structure: SupportStructureGene,
    pub position_to_id: HashMap<Point, SupportNodeId>,
    pub random: Random,
    pub settings: &'a Settings,
    pub supports: HashSet<Point>,
}

impl<'a> RawStructureBuilder<'a> {
    pub fn new(
        settings: &'a Settings,
        optimization_structure: &'a SupportStructureOptimizationGene,
    ) -> Self {
        // node zero is added as a placeholder, so that the we can use it for anchors, and then
        // remove it and repairing the structure
        let mut raw_structure = SupportStructureGene {
            nodes: Default::default(),
        };
        raw_structure.nodes.insert(
            SupportNodeId(0),
            SupportNode::Base(BaseNode {
                id: SupportNodeId(0),
                mesh_contact: None,
                last_position: Point::DOWNWARD.to_scaled(10e20),
                radius: 3.,
            }),
        );

        Self {
            raw_structure,
            optimization_structure,
            position_to_id: Default::default(),
            // random has no actual effect on the shape of the created structure
            // it only effect IDs, so we can put an unseeded value here
            random: Random::UnSeededRandom,
            settings,
            supports: HashSet::new(),
        }
    }

    pub fn build(mut self, graph: &SurfaceGraph) -> SupportStructureGene {
        self.insert_points();
        self.raw_structure.nodes.remove(&SupportNodeId(0));
        // the structure passed will already be completed, so the random will
        // have no effects. The repair will only fix-up the contacts
        // to the ground and the anchors
        self.raw_structure.repair(graph, &self.random);
        self.raw_structure
    }

    fn insert_points(&mut self) {
        let mut elements = vec![];
        let contacts_set: HashSet<_> = self
            .optimization_structure
            .contacts
            .iter()
            .map(|x| x.position)
            .collect();
        let mut contacts: Vec<_> = self
            .optimization_structure
            .contacts
            .iter()
            .map(|x| QueuedPoint::Contact(*x))
            .collect();
        let mut supports: Vec<_> = self
            .optimization_structure
            .supports
            .iter()
            .filter(|x| !contacts_set.contains(&x.position))
            .map(|x| QueuedPoint::Support(*x))
            .collect();
        elements.append(&mut contacts);
        elements.append(&mut supports);

        // sorting with the lowest noes at the bottom
        elements.sort_by_key(|x| Cost::new(-x.height()));

        while let Some(p) = elements.pop() {
            self.create_node(&p);
            let position = p.position();
            let supporters = self.find_supporters(p.position(), p.num_contacts());
            for s in supporters {
                self.add_connection(position, s);
            }
        }
    }

    fn create_node(&mut self, node: &QueuedPoint) -> SupportNodeId {
        let position = node.position();
        if let Some(k) = self.position_to_id.get(&position) {
            return *k;
        }
        let id = self.raw_structure.new_random_id(&self.random);
        let n = match node {
            QueuedPoint::Contact(p) => {
                let n = ContactNode {
                    id,
                    position,
                    radius: p.radius,
                    leans_on: vec![],
                };
                SupportNode::Contact(n)
            }
            QueuedPoint::Support(s) => {
                self.supports.insert(position);
                let n = MiddleNode {
                    id,
                    // we put a random anchor
                    anchor: PositionAnchor::new(SupportNodeId(0), Point::ZERO, Point::ZERO),
                    last_position: position,
                    leans_on: vec![],
                    radius: s.radius(self.optimization_structure),
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

    fn find_supporters(&self, position: Point, num_contacts: usize) -> Vec<Point> {
        let valid_points: Vec<_> = self
            .supports
            .iter()
            .filter(|x| {
                // todo: hardcoded value
                Point::horizon_angle(**x, position) > 30.0_f32.to_radians()
                // x.z < position.z
                && **x != position
            })
            .map(|x| {
                let distance = (*x - position).abs();
                (x, Cost::new(distance))
            })
            .collect();

        return valid_points
            .iter()
            .sorted_by_key(|x| x.1)
            .take(num_contacts)
            .map(|x| *x.0)
            .unique()
            .collect();
    }
}

enum QueuedPoint {
    Contact(ContactPoint),
    Support(SupportPoint),
}
impl QueuedPoint {
    pub fn position(&self) -> Point {
        match self {
            QueuedPoint::Contact(p) => p.position,
            QueuedPoint::Support(p) => p.position,
        }
    }
    pub fn height(&self) -> f32 {
        self.position().z
    }
    pub fn num_contacts(&self) -> usize {
        match self {
            QueuedPoint::Contact(_) => 1,
            QueuedPoint::Support(p) => p.num_contacts as usize,
        }
    }
}
