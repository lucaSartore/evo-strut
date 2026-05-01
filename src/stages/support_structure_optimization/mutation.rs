
use crate::{evolution::{Mutator, Random}, models::{Settings, SurfaceGraph}, stages::support_structure_optimization::SupportGroup};

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
pub mod edit_layer_connections;
pub mod move_layer_height;
pub mod move_points_in_layer;
pub mod remove_layer_point;
pub mod regenerate_group;


pub struct SupportStructureMutator<'a> {
    pub settings: &'a Settings,
    pub graph: &'a SurfaceGraph,
    pub rand: Random
}

impl<'a> Mutator<SupportGroup, SupportStructureMutatorSettings<'a>> for SupportStructureMutator<'a> {
    fn new(settings: &SupportStructureMutatorSettings<'a>, rand: Random) -> Self {
        Self {
            settings: settings.settings,
            graph: settings.graph,
            rand
        }
    }

    fn mutate(&self, gene: &mut SupportGroup) {
        enum MK {
            AddLayer,
            AddLayerPoint,
            EditLayerConnections,
            MoveLayerHeight,
            MovePointsInLayer,
            RegenerateGroup,
            RemoveLayerPoint,
        }
        const OPTIONS: &[MK] = &[
            MK::AddLayer,
            MK::AddLayerPoint,
            MK::EditLayerConnections,
            MK::MoveLayerHeight,
            MK::MovePointsInLayer,
            MK::RemoveLayerPoint,
            MK::RegenerateGroup,
        ];

        let n_mutations = self.rand.next_in_range(1, 3);
        for _ in 0..n_mutations {
            let mutation = self.rand.choose_or_panic(OPTIONS);

            match mutation {
                MK::AddLayer => add_layer::mutate(self, gene),
                MK::AddLayerPoint => add_layer_point::mutate(self, gene),
                MK::EditLayerConnections => edit_layer_connections::mutate(self, gene),
                MK::MoveLayerHeight => move_layer_height::mutate(self, gene),
                MK::MovePointsInLayer => move_points_in_layer::mutate(self, gene),
                MK::RemoveLayerPoint => remove_layer_point::mutate(self, gene),
                MK::RegenerateGroup => regenerate_group::mutate(self, gene),
            };
        }
    }
}
