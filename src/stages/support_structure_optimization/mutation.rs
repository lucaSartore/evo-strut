
use crate::{evolution::{Mutator, Random}, models::{Settings, SurfaceGraph}, stages::support_structure_optimization::models::CompressedSupportGene};

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

pub mod add_layer;
pub mod add_layer_point;
pub mod create_new_group;
pub mod edit_layer_connections;
pub mod move_contact;
pub mod move_layer_height;
pub mod move_points_in_layer;
pub mod remove_layer_point;


pub struct SupportStructureMutator<'a> {
    pub settings: &'a Settings,
    pub graph: &'a SurfaceGraph,
    pub rand: Random
}

impl<'a> Mutator<CompressedSupportGene, SupportStructureMutatorSettings<'a>> for SupportStructureMutator<'a> {
    fn new(settings: &SupportStructureMutatorSettings<'a>, rand: Random) -> Self {
        Self {
            settings: settings.settings,
            graph: settings.graph,
            rand
        }
    }

    fn mutate(&self, gene: &mut CompressedSupportGene) {
        enum MK {
            AddLayer,
            AddLayerPoint,
            CreateNewGroup,
            EditLayerConnections,
            MoveContact,
            MoveLayerHeight,
            MovePointsInLayer,
            RemoveLayerPoint,
        }
        const OPTIONS: &[MK] = &[
            MK::AddLayer,
            // MK::AddLayerPoint,
            // MK::CreateNewGroup,
            // MK::EditLayerConnections,
            MK::MoveContact,
            // MK::MoveLayerHeight,
            // MK::MovePointsInLayer,
            // MK::RemoveLayerPoint
        ];

        let n_mutations = self.rand.next_in_range(1, 3);
        for _ in 0..n_mutations {
            let mutation = self.rand.choose_or_panic(OPTIONS);

            match mutation {
                MK::AddLayer => add_layer::mutate(self, gene),
                MK::AddLayerPoint => add_layer_point::mutate(self, gene),
                MK::CreateNewGroup => create_new_group::mutate(self, gene),
                MK::EditLayerConnections => edit_layer_connections::mutate(self, gene),
                MK::MoveContact => move_contact::mutate(self, gene),
                MK::MoveLayerHeight => move_layer_height::mutate(self, gene),
                MK::MovePointsInLayer => move_points_in_layer::mutate(self, gene),
                MK::RemoveLayerPoint => remove_layer_point::mutate(self, gene)
            };
        }
    }
}
