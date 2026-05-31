use crate::stages::support_structure_refinement::evaluation::SupportStructureEvaluator as FullEvaluator;
pub use crate::stages::support_structure_refinement::evaluation::SupportStructureEvaluatorSettings;
use crate::{
    evolution::{Cost, Evaluator},
    stages::support_structure_optimization::SupportStructureOptimizationGene,
};

pub struct SupportStructureEvaluator<'a> {
    evaluator: FullEvaluator<'a>,
}

impl<'a> Evaluator<SupportStructureOptimizationGene, SupportStructureEvaluatorSettings<'a>>
    for SupportStructureEvaluator<'a>
{
    fn new(settings: &SupportStructureEvaluatorSettings<'a>) -> Self {
        Self {
            evaluator: FullEvaluator::new(settings),
        }
    }

    fn evaluate(&self, gene: &SupportStructureOptimizationGene) -> Cost {
        let gene = gene.to_full_gene(self.evaluator.graph, self.evaluator.settings);
        let cost = self.evaluator.evaluate(&gene);
        cost
    }

    fn visualize(&self, gene: &SupportStructureOptimizationGene) -> anyhow::Result<()> {
        self.evaluator
            .visualize(&gene.to_full_gene(&self.evaluator.graph, self.evaluator.settings))
    }
}
