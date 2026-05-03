use rerun::RecordingStream;

use crate::{
    evolution::Evaluator,
    models::{Settings, SurfaceGraph},
    stages::support_structure_refinement::SupportStructureGene,
};

mod graph;
mod logic;
mod stiffness;
mod visualization;

pub struct SupportStructureEvaluatorSettings<'a> {
    pub settings: &'a Settings,
    pub graph: &'a SurfaceGraph,
}

impl<'a> SupportStructureEvaluatorSettings<'a> {
    pub fn new(settings: &'a Settings, graph: &'a SurfaceGraph) -> Self {
        Self { settings, graph }
    }
}

pub struct SupportStructureEvaluator<'a> {
    settings: &'a Settings,
    pub graph: &'a SurfaceGraph,
    stream: RecordingStream,
}

impl<'a> Evaluator<SupportStructureGene, SupportStructureEvaluatorSettings<'a>>
    for SupportStructureEvaluator<'a>
{
    fn new(settings: &SupportStructureEvaluatorSettings<'a>) -> Self {
        Self {
            settings: settings.settings,
            graph: settings.graph,
            stream: rerun::RecordingStreamBuilder::new("contact points structure optimization")
                .spawn()
                .expect("fail to build rerun stream"),
        }
    }

    fn evaluate(&self, gene: &SupportStructureGene) -> crate::evolution::Cost {
        logic::evaluate_cost(gene, self.settings)
    }

    fn visualize(&self, gene: &SupportStructureGene) -> anyhow::Result<()> {
        visualization::visualize(&self.stream, gene, self.graph)
    }
}
