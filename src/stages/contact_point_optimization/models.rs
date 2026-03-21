use std::collections::HashSet;
use crate::{models::FaceId, support::links::Links};



#[derive(Debug, Clone, Default)]
pub struct ContactPointsGene {
    links: Links<FaceId, 3>,
    contact_points: HashSet<FaceId>
}

impl ContactPointsGene {
    pub fn merge_many(elements: Vec<Self>) -> Option<Self> {
        elements.into_iter().reduce(|mut a, b| {
            a.merge_with(b);
            a
        })
    }
    pub fn merge_with(&mut self, other: Self) {
        self.links.merge(&other.links);
        for c in other.contact_points {
            self.contact_points.insert(c);
        }
    }

    pub fn add_contact_point(&mut self, s: FaceId) {
        self.contact_points.insert(s);
    }

    pub fn if_supported(&self, id: FaceId) -> bool {
        self.contact_points.contains(&id)
    }

    pub fn iter_contacts(&self) -> impl Iterator<Item = &FaceId> {
        self.contact_points.iter()
    }

    pub fn num_contacts(&self) -> usize {
        self.contact_points.len()
    }

    pub fn iter_links(&self) -> impl Iterator<Item = (FaceId, FaceId)> {
        self.links.all_links()
    }
}
