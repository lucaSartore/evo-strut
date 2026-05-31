use crate::{
    evolution::PopulationInitializer,
    models::{Settings, SurfaceGraph},
    stages::support_structure_optimization::{mutation::SupportStructureMutator, SupportStructureOptimizationGene},
};

pub struct SupportStructureInitializerSettings<'a> {
    settings: &'a Settings,
    graph: &'a SurfaceGraph,
    group_template: &'a SupportStructureOptimizationGene,
}

impl<'a> SupportStructureInitializerSettings<'a> {
    pub fn new(
        settings: &'a Settings,
        graph: &'a SurfaceGraph,
        group_template: &'a SupportStructureOptimizationGene,
    ) -> Self {
        Self {
            settings,
            graph,
            group_template,
        }
    }
}

pub struct SupportStructureInitializer<'a> {
    settings: &'a Settings,
    group_template: &'a SupportStructureOptimizationGene,
    mutator: SupportStructureMutator<'a>,
}

impl<'a> PopulationInitializer<SupportStructureOptimizationGene, SupportStructureInitializerSettings<'a>>
    for SupportStructureInitializer<'a>
{
    fn new(
        settings: &SupportStructureInitializerSettings<'a>,
        rand: crate::evolution::Random,
    ) -> Self {
        Self {
            settings: settings.settings,
            group_template: settings.group_template,
            mutator: SupportStructureMutator {
                settings: settings.settings,
                graph: settings.graph,
                rand,
            },
        }
    }

    fn get_initial_individuals(&self) -> usize {
        self.settings
            .support_structure_optimization_settings
            .generation_size
    }

    fn get_random_individual(&self) -> SupportStructureOptimizationGene {
        let mut g = self.group_template.clone();
        // todo: add points at random
        g
    }
}
