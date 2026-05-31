use crate::{
    evolution::{Mutator, Random},
    models::{Settings, SurfaceGraph},
    stages::support_structure_optimization::SupportStructureOptimizationGene,
};

pub struct SupportStructureMutatorSettings<'a> {
    settings: &'a Settings,
    graph: &'a SurfaceGraph,
}

impl<'a> SupportStructureMutatorSettings<'a> {
    pub fn new(settings: &'a Settings, graph: &'a SurfaceGraph) -> Self {
        Self { settings, graph }
    }
}

pub mod add_point;
pub mod move_points;
pub mod mutate_network;
pub mod remove_point;

pub struct SupportStructureMutator<'a> {
    pub settings: &'a Settings,
    pub graph: &'a SurfaceGraph,
    pub rand: Random,
}

impl<'a> Mutator<SupportStructureOptimizationGene, SupportStructureMutatorSettings<'a>>
    for SupportStructureMutator<'a>
{
    fn new(settings: &SupportStructureMutatorSettings<'a>, rand: Random) -> Self {
        Self {
            settings: settings.settings,
            graph: settings.graph,
            rand,
        }
    }

    fn mutate(&self, gene: &mut SupportStructureOptimizationGene) {
        enum MK {
            AddPont,
            MovePoints,
            MutateNetwork,
            RemovePoint,
        }
        const OPTIONS: &[MK] = &[
            MK::AddPont,
            MK::MovePoints,
            // MK::MutateNetwork,
            MK::RemovePoint,
        ];

        let n_mutations = self.rand.next_in_range(1, 3);
        for _ in 0..n_mutations {
            let mutation = self.rand.choose_or_panic(OPTIONS);

            match mutation {
                MK::AddPont => add_point::mutate(self, gene),
                MK::MovePoints => move_points::mutate(self, gene),
                MK::MutateNetwork => mutate_network::mutate(self, gene),
                MK::RemovePoint => remove_point::mutate(self, gene),
            };
        }
    }
}
