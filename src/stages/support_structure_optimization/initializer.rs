use hashbrown::HashMap;
use smallvec::smallvec;

use crate::{evolution::{PopulationInitializer, Random}, models::{Settings, SurfaceGraph}, stages::{contact_point_optimization::ContactPointsGene, support_structure_optimization::{ContactNode, SupportNode, SupportStructureGene}}};


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
    graph: &'a SurfaceGraph,
    rand:  Random,
    empty_gene: SupportStructureGene
}

impl<'a> PopulationInitializer<SupportStructureGene, SupportStructureInitializerSettings<'a>> for SupportStructureInitializer<'a> {
    fn new(settings: &SupportStructureInitializerSettings<'a>, rand: crate::evolution::Random) -> Self {
        let mut empty_gene = SupportStructureGene { nodes: HashMap::default() };
        for (contact_point_id, contact_point_shape) in &settings.contact_points.contact_points {
            let id = empty_gene.new_random_id(&rand);
            let position = settings.graph.get_triangle(*contact_point_id).center();
            let c = ContactNode{
                id,
                position,
                radius: contact_point_shape.radius,
                leans_on: smallvec![]
            };
            empty_gene.nodes.insert(id, SupportNode::Contact(c));
        }
        SupportStructureInitializer { 
            settings: settings.settings,
            graph: settings.graph,
            rand,
            empty_gene
        }
    }

    fn get_initial_individuals(&self) -> usize {
        self.settings.support_structure_optimization_settings.generation_size
    }

    fn get_random_individual(&self) -> SupportStructureGene {
        let mut s = self.empty_gene.clone();
        s.repair(self.graph, &self.rand);
        s
    }
}
