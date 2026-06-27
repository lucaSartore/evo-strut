use baby_shark::{
    mesh::corner_table::{CornerTable, CornerTableF},
    voxel::{prelude::MeshToVolume, volume::Volume},
};
use rerun::RecordingStream;

use crate::{
    evolution::Evaluator,
    models::{Settings, SurfaceGraph},
    stages::{
        floating_region_detection::FloatingRegion, support_structure_optimization::SupportStructureOptimizationGene
    },
};

pub mod graph;
pub mod logic;
mod stiffness;
pub mod visualization;


pub struct SupportStructureEvaluatorSettings<'a> {
    pub settings: &'a Settings,
    pub graph: &'a SurfaceGraph,
    pub mesh: &'a CornerTableF,
    pub floating_region: Vec<FloatingRegion>,
}
impl<'a> SupportStructureEvaluatorSettings<'a> {
    pub fn new(settings: &'a Settings, graph: &'a SurfaceGraph, mesh: &'a CornerTable<f32>, floating_region: Vec<FloatingRegion>) -> Self {
        Self { settings, graph, mesh, floating_region }
    }
}

pub struct SupportStructureEvaluator<'a> {
    pub settings: &'a Settings,
    pub graph: &'a SurfaceGraph,
    volume: Volume,
    stream: RecordingStream,
    floating_regions: Vec<FloatingRegion>,
}

impl<'a> Evaluator<SupportStructureOptimizationGene, SupportStructureEvaluatorSettings<'a>>
    for SupportStructureEvaluator<'a>
{
    fn new(settings: &SupportStructureEvaluatorSettings<'a>) -> Self {
        let s = &settings.settings.support_structure_cost_settings;
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
            volume,
            floating_regions: settings.floating_region.clone(),
        }
    }

    fn evaluate(&self, gene: &SupportStructureOptimizationGene) -> crate::evolution::Cost {
        logic::evaluate_cost(
            gene,
            self.graph,
            &self.volume,
            self.settings,
            &self.floating_regions,
        )
    }

    fn visualize(&self, gene: &SupportStructureOptimizationGene) -> anyhow::Result<()> {
        visualization::visualize(&self.stream, gene, self.graph, self.settings)
    }
}
