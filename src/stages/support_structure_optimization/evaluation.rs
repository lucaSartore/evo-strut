use crate::{evolution::Evaluator, models::Settings, stages::support_structure_optimization::SupportStructureGene};


pub struct SupportStructureEvaluatorSettings<'a> {
    settings: &'a Settings
}

impl<'a> SupportStructureEvaluatorSettings<'a> {
    pub fn new(settings: &'a Settings) -> Self {
        Self {
            settings
        }
    }
}

pub struct SupportStructureEvaluator<'a> {
    settings: &'a Settings
}

impl<'a> Evaluator<SupportStructureGene, SupportStructureEvaluatorSettings<'a>> for SupportStructureEvaluator<'a> {
    fn new(settings: &SupportStructureEvaluatorSettings<'a>) -> Self {
        todo!()
    }

    fn evaluate(&self, gene: &SupportStructureGene) -> crate::evolution::Cost {
        todo!()
    }

    fn visualize(&self, gene: &SupportStructureGene) -> anyhow::Result<()> {
        todo!()
    }
}
