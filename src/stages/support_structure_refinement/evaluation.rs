use baby_shark::{mesh::corner_table::CornerTableF, voxel::{prelude::MeshToVolume, volume::Volume}};
use rerun::RecordingStream;

use crate::{
    evolution::Evaluator,
    models::{Settings, SurfaceGraph},
    stages::support_structure_refinement::SupportStructureGene,
};

mod graph;
pub mod logic;
mod stiffness;
mod visualization;

pub struct SupportStructureEvaluatorSettings<'a> {
    pub settings: &'a Settings,
    pub graph: &'a SurfaceGraph,
    pub mesh: &'a CornerTableF
}

impl<'a> SupportStructureEvaluatorSettings<'a> {
    pub fn new(settings: &'a Settings, graph: &'a SurfaceGraph, mesh: &'a CornerTableF) -> Self {
        Self { settings, graph, mesh }
    }
}

pub struct SupportStructureEvaluator<'a> {
    pub settings: &'a Settings,
    pub graph: &'a SurfaceGraph,
    volume: Volume,
    stream: RecordingStream,
}

impl<'a> Evaluator<SupportStructureGene, SupportStructureEvaluatorSettings<'a>>
    for SupportStructureEvaluator<'a>
{
    fn new(settings: &SupportStructureEvaluatorSettings<'a>) -> Self {
        let s = &settings.settings.support_structure_refinement_settings;
        let volume = MeshToVolume::default()
            .with_voxel_size(s.collision_volume_voxel_size)
            .convert(settings.mesh)
            .expect("conversion to volume fail")
            .offset(s.collision_volume_offset);

        Self {
            settings: settings.settings,
            graph: settings.graph,
            stream: rerun::RecordingStreamBuilder::new("contact points structure optimization")
                .spawn()
                .expect("fail to build rerun stream"),
            volume
        }
    }

    fn evaluate(&self, gene: &SupportStructureGene) -> crate::evolution::Cost {
        logic::evaluate_cost(gene, &self.volume, self.settings)

    }

    fn visualize(&self, gene: &SupportStructureGene) -> anyhow::Result<()> {
        visualization::visualize(&self.stream, gene, self.graph)
    }
}
