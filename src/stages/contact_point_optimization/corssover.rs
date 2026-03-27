
use std::ops::Deref;

use rerun::demo_util::grid;

use crate::models::{FaceId, Plane, Settings, SurfaceGraph};
use crate::evolution::*;
use super::models::ContactPointsGene;

pub struct ContactPointCrossoverSettings<'a> {
    settings: &'a Settings,
    available_options: &'a[FaceId],
    graph: &'a SurfaceGraph
}

impl<'a> ContactPointCrossoverSettings<'a> {
    pub fn new(settings: &'a Settings, available_options: &'a[FaceId], graph: &'a SurfaceGraph) -> Self {
        Self{
            settings,
            available_options,
            graph
        }
    }
}


pub struct ContactPointCrossover<'a>{
    settings: &'a Settings,
    available_options: &'a[FaceId],
    graph: &'a SurfaceGraph,
    rand: Random
}

impl<'a> Crossover<ContactPointsGene, ContactPointCrossoverSettings<'a>> for ContactPointCrossover<'a> {
    fn new(settings: &ContactPointCrossoverSettings<'a>, rand: Random) -> Self {
        Self {
            settings: settings.settings,
            available_options: settings.available_options,
            graph: settings.graph,
            rand
        }
    }

    fn crossover(&self, a: &ContactPointsGene, b: &ContactPointsGene) -> ContactPointsGene {
        // if the optimized area is too small, we can't do any meaningful crossover... we
        // just pick random
        if self.available_options.len() < 10 {
            let options = [a,b];
            let e = self.rand.choose_or_panic(&options);
            return (*e).clone();
        }

        // pick 3 random points to generate a plane
        let options = self.rand.choose_many(3, self.available_options);
        let (p1_id, p2_id, p3_id) = (*options[0], *options[1], *options[2]);
        let (p1, p2, p3) = (self.graph.get_triangle(p1_id).center(), self.graph.get_triangle(p2_id).center(), self.graph.get_triangle(p3_id).center());

        let plane = Plane::from_points_and_max_distance(p1, p2, p3);

        let mut new_gene = ContactPointsGene::default();

        for (id, shape) in a.iter_contacts() {
            let p = self.graph.get_triangle(*id).center();
            if plane.classify_point(p) {
                new_gene.add_contact_point(*id, *shape);
            }
        }
        for (id, shape) in b.iter_contacts() {
            let p = self.graph.get_triangle(*id).center();
            if !plane.classify_point(p) {
                new_gene.add_contact_point(*id, *shape);
            }
        }

        new_gene
    }
}
