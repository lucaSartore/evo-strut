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
        Cost::new(gene
            .to_full_genes(&self.evaluator.graph)
            .iter()
            .map(|x| self.evaluator.evaluate(x).as_f32())
            .sum())
    }

    fn visualize(&self, gene: &CompressedSupportGene) -> anyhow::Result<()> {
        self.evaluator.visualize(&gene.to_full_gene(&self.evaluator.graph))
    }
}
