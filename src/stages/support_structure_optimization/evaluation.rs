use crate::{evolution::{Cost, Evaluator}, stages::support_structure_optimization::models::CompressedSupportGene};
use crate::stages::support_structure_refinement::evaluation::SupportStructureEvaluator as FullEvaluator;
pub use crate::stages::support_structure_refinement::evaluation::SupportStructureEvaluatorSettings;

pub struct SupportStructureEvaluator<'a> {
    evaluator: FullEvaluator<'a>
}

impl<'a> Evaluator<CompressedSupportGene, SupportStructureEvaluatorSettings<'a>> for SupportStructureEvaluator<'a> {
    fn new(settings: &SupportStructureEvaluatorSettings<'a>) -> Self {
        Self {
            evaluator: FullEvaluator::new(settings)
        }
    }

    fn evaluate(&self, gene: &CompressedSupportGene) -> Cost {
        let gene = gene.to_full_gene(self.evaluator.graph);
        let cost = self.evaluator.evaluate(&gene);
        cost
    }

    fn visualize(&self, gene: &CompressedSupportGene) -> anyhow::Result<()> {
        self.evaluator.visualize(&gene.to_full_gene(&self.evaluator.graph))
    }
}
