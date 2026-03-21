use core::num;

use crate::{evolution::{PopulationInitializer, Random}, models::{FaceId, Settings, SurfaceGraph}, stages::contact_point_optimization::models::ContactPointsGene};

pub struct ContactPointsInitializerSettings<'a> {
    settings: &'a Settings,
    graph: &'a SurfaceGraph,
    options: &'a [FaceId]
}

impl<'a> ContactPointsInitializerSettings<'a> {
    pub fn new(settings: &'a Settings, graph: &'a SurfaceGraph, options: &'a [FaceId]) -> Self {
        Self {
            settings,
            graph,
            options
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
        self.settings.contact_points_optimization_settings.population_size
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
            g.add_contact_point(*s);
        }
        g
    }
}
