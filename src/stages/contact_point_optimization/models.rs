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
}
