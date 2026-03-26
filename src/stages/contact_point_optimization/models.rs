use core::f32;

use crate::{evolution::Random, models::{Face, FaceId, Settings, SurfaceGraph}, support::{graph_circle::find_circle, links::Links}};
use hashbrown::{HashMap, HashSet};


#[derive(Debug, Clone, Default)]
pub struct ContactPointShape {
    pub radius: f32
}

impl ContactPointShape {
    pub fn random(rand: &Random, settings: &Settings) -> Self {
        Self {
            radius: rand.next_f32(0., settings.contact_points_optimization_settings.max_support_radius)
        }
    }

    pub fn area(&self) -> f32 {
        self.radius.powi(2) * f32::consts::PI
    }
}

#[derive(Debug, Clone, Default)]
pub struct ContactPointsGene {
    pub contact_points: HashMap<FaceId, ContactPointShape>
}

impl ContactPointsGene {
    pub fn merge_many(elements: Vec<Self>) -> Option<Self> {
        elements.into_iter().reduce(|mut a, b| {
            a.merge_with(b);
            a
        })
    }
    pub fn merge_with(&mut self, other: Self) {
        for c in other.contact_points {
            self.contact_points.insert(c.0, c.1);
        }
    }

    pub fn add_contact_point(&mut self, face: FaceId, shape: ContactPointShape) {
        self.contact_points.insert(face, shape);
    }

    // pub fn is_supported(&self, id: FaceId) -> bool {
    //     self.contact_points.contains(&id)
    // }
    
    pub fn get_supported(&self, graph: &SurfaceGraph) -> HashSet<FaceId> {
        let mut to_return = HashSet::new();
        for (center, shape) in self.contact_points.iter() {
            let c = find_circle(graph, *center, shape.radius);
            to_return.extend(c);
        }
        to_return
    }

    pub fn iter_contacts(&self) -> impl Iterator<Item = (&FaceId, &ContactPointShape)> {
        self.contact_points.iter()
    }

    pub fn num_contacts(&self) -> usize {
        self.contact_points.len()
    }

}
