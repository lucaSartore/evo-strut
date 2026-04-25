use core::f32;

use itertools::{Group, Itertools};
use rerun::external::glam::usize;

use crate::{evolution::{PopulationInitializer, Random}, models::{Settings, SurfaceGraph}, stages::{contact_point_optimization::ContactPointsGene, support_structure_optimization::{CompressedSupportGene, models::{ContactPoint, SupportGroup}}}};


pub struct SupportStructureInitializerSettings<'a> {
    settings: &'a Settings,
    contact_points: &'a ContactPointsGene,
    graph: &'a SurfaceGraph
}

impl<'a> SupportStructureInitializerSettings<'a> {
    pub fn new(settings: &'a Settings, contact_points: &'a ContactPointsGene, graph: &'a SurfaceGraph) -> Self {
        Self {
            settings,
            contact_points,
            graph
        }
    }
}

pub struct SupportStructureInitializer<'a> {
    settings: &'a Settings,
    contact_points: &'a ContactPointsGene,
    graph: &'a SurfaceGraph,
    rand: Random
}

impl<'a> PopulationInitializer<CompressedSupportGene, SupportStructureInitializerSettings<'a>> for SupportStructureInitializer<'a> {
    fn new(settings: &SupportStructureInitializerSettings<'a>, rand: crate::evolution::Random) -> Self {
        Self {
            settings: settings.settings,
            contact_points: settings.contact_points,
            graph: settings.graph,
            rand
        }
    }

    fn get_initial_individuals(&self) -> usize {
        self.settings.support_structure_optimization_settings.generation_size
    }

    fn get_random_individual(&self) -> CompressedSupportGene {
        let mult = self.settings.support_structure_optimization_settings.num_initial_groups_multiplier;
        let num_groups = (mult * self.contact_points.contact_points.len() as f32) as usize;

        let groups = self.contact_points
            .contact_points
            .iter()
            .chunk_by(|_| self.rand.next_in_range(0, num_groups as u64))
            .into_iter()
            .map(|x| {
                SupportGroup {
                    supports: x.1.map(|y| ContactPoint{ 
                        position: self.graph.get_triangle(*y.0).center(),
                        radius: y.1.radius
                    }).collect(),
                    layers: vec![]
                }
            }).collect();
        CompressedSupportGene{ groups }
    }
}
