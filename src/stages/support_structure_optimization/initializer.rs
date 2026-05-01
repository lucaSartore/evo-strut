
use rerun::external::glam::usize;

use crate::{evolution::{PopulationInitializer, Random}, models::{Settings, SurfaceGraph}, stages::{contact_point_optimization::ContactPointsGene, contact_points_grouping::ContactPointGroupingGene, support_structure_optimization::CompressedSupportGene}};


pub struct SupportStructureInitializerSettings<'a> {
    settings: &'a Settings,
    contact_points: &'a ContactPointsGene,
    grouper: &'a ContactPointGroupingGene,
    graph: &'a SurfaceGraph
}

impl<'a> SupportStructureInitializerSettings<'a> {
    pub fn new(settings: &'a Settings, contact_points: &'a ContactPointsGene, grouper: &'a ContactPointGroupingGene, graph: &'a SurfaceGraph) -> Self {
        Self {
            settings,
            contact_points,
            grouper,
            graph
        }
    }
}

pub struct SupportStructureInitializer<'a> {
    settings: &'a Settings,
    contact_points: &'a ContactPointsGene,
    grouper: &'a ContactPointGroupingGene,
    graph: &'a SurfaceGraph,
    rand: Random
}

impl<'a> PopulationInitializer<CompressedSupportGene, SupportStructureInitializerSettings<'a>> for SupportStructureInitializer<'a> {
    fn new(settings: &SupportStructureInitializerSettings<'a>, rand: crate::evolution::Random) -> Self {
        Self {
            settings: settings.settings,
            contact_points: settings.contact_points,
            grouper: settings.grouper,
            graph: settings.graph,
            rand
        }
    }

    fn get_initial_individuals(&self) -> usize {
        self.settings.support_structure_optimization_settings.generation_size
    }

    fn get_random_individual(&self) -> CompressedSupportGene {
        return self.grouper.to_compressed_gene(
            self.contact_points,
            self.graph,
            self.settings,
            &self.rand
        )
    }
}
