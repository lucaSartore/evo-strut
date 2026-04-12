use crate::{evolution::{Mutator, Random}, models::{Settings, SurfaceGraph}, stages::support_structure_optimization::SupportStructureGene};


pub struct SupportStructureMutatorSettings<'a> {
    settings: &'a Settings,
    graph: &'a SurfaceGraph
}

impl<'a> SupportStructureMutatorSettings<'a> {
    pub fn new(settings: &'a Settings, graph: &'a SurfaceGraph) -> Self {
        Self {
            settings,
            graph
        }
    }
}


pub struct SupportStructureMutator<'a> {
    settings: &'a Settings,
    graph: &'a SurfaceGraph,
    rand: Random
}

impl<'a> Mutator<SupportStructureGene, SupportStructureMutatorSettings<'a>> for SupportStructureMutator<'a> {
    fn new(settings: &SupportStructureMutatorSettings<'a>, rand: Random) -> Self {
        Self {
            settings: settings.settings,
            graph: settings.graph,
            rand
        }
    }

    fn mutate(&self, gene: &mut SupportStructureGene) {
        // todo: implement real mutation
        gene.repair(&self.graph, &self.rand);
    }
}
