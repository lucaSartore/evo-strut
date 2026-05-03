use crate::stages::support_structure_refinement::evaluation::SupportStructureEvaluator as FullEvaluator;
pub use crate::stages::support_structure_refinement::evaluation::SupportStructureEvaluatorSettings;
use crate::{
    evolution::{Cost, Evaluator},
    stages::support_structure_optimization::SupportGroup,
};

pub struct SupportStructureEvaluator<'a> {
    evaluator: FullEvaluator<'a>,
}

impl<'a> Evaluator<SupportGroup, SupportStructureEvaluatorSettings<'a>>
    for SupportStructureEvaluator<'a>
{
    fn new(settings: &SupportStructureEvaluatorSettings<'a>) -> Self {
        Self {
            evaluator: FullEvaluator::new(settings),
        }
    }

    fn evaluate(&self, gene: &SupportGroup) -> Cost {
        let gene = gene.to_full_gene(self.evaluator.graph);
        let cost = self.evaluator.evaluate(&gene);
        cost
    }

    fn visualize(&self, gene: &SupportGroup) -> anyhow::Result<()> {
        self.evaluator
            .visualize(&gene.to_full_gene(&self.evaluator.graph))
    }
}
