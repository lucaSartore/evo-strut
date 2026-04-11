use crate::{evolution::Evaluator, models::{Settings, SurfaceGraph}, stages::support_structure_optimization::SupportStructureGene};

mod stiffness;

pub struct SupportStructureEvaluatorSettings<'a> {
    settings: &'a Settings,
    graph: &'a SurfaceGraph
}

impl<'a> SupportStructureEvaluatorSettings<'a> {
    pub fn new(settings: &'a Settings, graph: &'a SurfaceGraph) -> Self {
        Self {
            settings,
            graph
        }
    }
}

pub struct SupportStructureEvaluator<'a> {
    settings: &'a Settings,
    graph: &'a SurfaceGraph
}

impl<'a> Evaluator<SupportStructureGene, SupportStructureEvaluatorSettings<'a>> for SupportStructureEvaluator<'a> {
    fn new(settings: &SupportStructureEvaluatorSettings<'a>) -> Self {
        Self {
            settings: settings.settings,
            graph: settings.graph
        }
    }

    fn evaluate(&self, gene: &SupportStructureGene) -> crate::evolution::Cost {
        todo!()
    }

    fn visualize(&self, gene: &SupportStructureGene) -> anyhow::Result<()> {
        todo!()
    }
}
