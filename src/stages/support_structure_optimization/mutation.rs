use crate::{evolution::{Mutator, Random}, models::{Settings, SurfaceGraph}, stages::support_structure_optimization::SupportStructureGene};

mod merge_two;
mod move_one;
mod remove_one_node;
mod remove_one_leans_on;
mod add_one_leans_on;
mod create_one_more_leans_on;

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
        enum MK {
            AddOneLeansOn,
            CreateOneMoreLeansOn,
            MergeTwo,
            MoveOne,
            RemoveOneLeansOn,
            RemoveOneNode
        }
        const OPTIONS: &[MK] = &[
            MK::AddOneLeansOn,
            MK::CreateOneMoreLeansOn,
            MK::MergeTwo,
            MK::MoveOne,
            MK::RemoveOneLeansOn,
            MK::RemoveOneNode
        ];

        let mutation = self.rand.choose_or_panic(OPTIONS);

        match mutation {
            MK::AddOneLeansOn => add_one_leans_on::mutate(self, gene),
            MK::CreateOneMoreLeansOn => create_one_more_leans_on::mutate(self, gene),
            MK::MergeTwo => merge_two::mutate(self, gene),
            MK::MoveOne => move_one::mutate(self, gene),
            MK::RemoveOneLeansOn => remove_one_leans_on::mutate(self, gene),
            MK::RemoveOneNode => remove_one_node::mutate(self, gene),
        };
        gene.repair(self.graph, &self.rand);
    }
}
