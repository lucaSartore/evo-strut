use crate::{
    evolution::PopulationInitializer,
    models::{Settings, SurfaceGraph},
    stages::support_structure_optimization::{
        mutation::SupportStructureMutator, SupportPoint, SupportStructureOptimizationGene,
    },
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

impl<'a>
    PopulationInitializer<SupportStructureOptimizationGene, SupportStructureInitializerSettings<'a>>
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

        // randomly generating some points
        // todo: hardcoded value
        let support_density = 0.0001; // support point per mm^3
        let convex_hull = &g.convex_hull;
        let area = convex_hull.area();
        let height = g.max_height();
        let volume = area * height;

        let n_supports = (volume * support_density) as usize;

        let mut to_add = vec![];
        for _ in 0..n_supports {
            let p = convex_hull.random_point(&self.mutator.rand);
            let num_contacts = self.mutator.rand.next_u32() % 3 + 1;
            to_add.push(SupportPoint {
                position: p,
                num_contacts,
            });
        }
        g.supports.append(&mut to_add);
        g
    }
}
