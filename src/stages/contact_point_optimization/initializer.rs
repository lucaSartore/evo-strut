use core::num;

use hashbrown::HashSet;

use crate::{evolution::{PopulationInitializer, Random}, models::{FaceId, Settings, SurfaceGraph}, stages::contact_point_optimization::{ContactPointShape, models::ContactPointsGene}};

pub struct ContactPointsInitializerSettings<'a> {
    pub settings: &'a Settings,
    pub graph: &'a SurfaceGraph,
    pub options: &'a [FaceId],
    pub options_hash: &'a HashSet<FaceId>
}

impl<'a> ContactPointsInitializerSettings<'a> {
    pub fn new(settings: &'a Settings, graph: &'a SurfaceGraph, options: &'a [FaceId], options_hash: &'a HashSet<FaceId>) -> Self {
        Self {
            settings,
            graph,
            options,
            options_hash
        }
    }
    
}

pub struct  ContactPointInitializer<'a> {
    settings: &'a Settings,
    graph: &'a SurfaceGraph,
    options: &'a [FaceId],
    rand: Random
}

impl<'a> PopulationInitializer<ContactPointsGene, ContactPointsInitializerSettings<'a>> for ContactPointInitializer<'a> {
    fn new(settings: &ContactPointsInitializerSettings<'a>, rand: Random) -> Self {
        Self {
            settings: settings.settings,
            options: settings.options,
            graph: settings.graph,
            rand
        }
    }

    fn get_initial_individuals(&self) -> usize {
        self.settings.contact_points_optimization_settings.generation_size
    }

    fn get_random_individual(&self) -> ContactPointsGene {
        let mut g = ContactPointsGene::default();
        let area: f32 = self
            .options
            .iter()
            .map(|x| self.graph.get_triangle(*x).area())
            .sum();
        let support_density = &self.settings.contact_points_optimization_settings.initialization_support_density;
        let num_supports = (area * self.rand.next_distribution(support_density)) as usize;
        for _ in 0..num_supports {
            let s = self.rand.choose(self.options).expect("options shall always be at least one");
            let r = ContactPointShape::random(&self.rand, self.settings);
            g.add_contact_point(*s, r);
        }
        g
    }
}
