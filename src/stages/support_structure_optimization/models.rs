use core::f32;
use std::fmt::Debug;

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    evolution::{Cost, Random},
    models::{FaceId, Point, Settings, SurfaceGraph},
    stages::support_structure_optimization::evaluation::logic::GraphDescriptor,
    support::convex_hull::ConvexHull,
};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SupportNodeId(pub u32);

#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct ContactPoint {
    pub face: FaceId,
    pub position: Point,
    pub radius: f32,
}

#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct SupportPoint {
    pub position: Point,
    pub num_contacts: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SupportStructureOptimizationGene {
    pub contacts: Vec<ContactPoint>,
    pub supports: Vec<SupportPoint>,
    pub convex_hull: ConvexHull,
}

impl SupportStructureOptimizationGene {
    pub fn from_contacts(contacts: Vec<ContactPoint>) -> Self {
        let convex_hull = ConvexHull::new(contacts.iter().map(|x| x.position).collect());
        Self {
            contacts,
            supports: vec![],
            convex_hull,
        }
    }

    pub fn to_graph_descriptor(
        &self,
        graph: &SurfaceGraph,
        settings: &Settings,
    ) -> GraphDescriptor {
        let builder = RawGraphBuilder::new(settings, self);
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

struct RawGraphBuilder<'a> {
    pub optimization_structure: &'a SupportStructureOptimizationGene,
    pub raw_structure: GraphDescriptor,
    pub position_to_id: HashMap<Point, SupportNodeId>,
    pub random: Random,
    pub settings: &'a Settings,
    pub supports: HashSet<Point>,
}

impl<'a> RawGraphBuilder<'a> {
    pub fn new(
        settings: &'a Settings,
        optimization_structure: &'a SupportStructureOptimizationGene,
    ) -> Self {
        Self {
            raw_structure: GraphDescriptor::default(),
            optimization_structure,
            position_to_id: Default::default(),
            // random has no actual effect on the shape of the created structure
            // it only effect IDs, so we can put an unseeded value here
            random: Random::UnSeededRandom,
            settings,
            supports: HashSet::new(),
        }
    }

    pub fn build(mut self, graph: &SurfaceGraph) -> GraphDescriptor {
        self.insert_points(graph);
        self.raw_structure.sort();
        self.raw_structure
    }

    fn insert_points(&mut self, _graph: &SurfaceGraph) {
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
        let (position, contact, radius) = match node {
            QueuedPoint::Contact(p) => (p.position, true, p.radius),
            QueuedPoint::Support(s) => {
                self.supports.insert(position);
                (
                    s.position,
                    false,
                    self.settings.support_settings.beam_radius,
                )
            }
        };

        self.add_node(position, contact, radius)
    }

    pub fn add_node(&mut self, position: Point, contact: bool, radius: f32) -> SupportNodeId {
        let id = self.raw_structure.new_random_id(&self.random);
        let supported = position.z == 0.;
        self.raw_structure
            .add_node(id, position, supported, contact, radius);
        self.position_to_id.insert(position, id);
        id
    }

    pub fn add_connection(&mut self, from: Point, to: Point) {
        let from_id = self.position_to_id[&from];
        let to_id = self.position_to_id[&to];
        self.raw_structure.add_link(from_id, to_id);
    }

    fn find_supporters(&mut self, position: Point, num_contacts: usize) -> Vec<Point> {
        let angle_threshold = self
            .settings
            .support_structure_optimization_settings
            .max_support_angle;
        let valid_points: Vec<_> = self
            .supports
            .iter()
            .filter(|x| {
                Point::horizon_angle(**x, position) > angle_threshold.to_radians()
                    && **x != position
            })
            .map(|x| {
                let distance = (*x - position).abs();
                (x, Cost::new(distance))
            })
            .collect();

        let p: Vec<Point> = valid_points
            .iter()
            .sorted_by_key(|x| x.1)
            .take(num_contacts)
            .map(|x| *x.0)
            .unique()
            .collect();

        if !p.is_empty() {
            return p;
        }

        let mut new_position = position;
        new_position.z = 0.;
        let radius = self.settings.support_settings.beam_radius;
        let id = self.add_node(new_position, false, radius);
        vec![self.raw_structure.details[&id].position]
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
