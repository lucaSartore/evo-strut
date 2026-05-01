use crate::{stages::support_structure_refinement::evaluation::SupportStructureEvaluator as FullEvaluator, support::convex_hull::ConvexHull};
pub use crate::stages::support_structure_refinement::evaluation::SupportStructureEvaluatorSettings;
use crate::{
    evolution::{Cost, Evaluator, Random},
    models::{Settings, SurfaceGraph},
    stages::{
        contact_point_optimization::ContactPointsGene,
        contact_points_grouping::models::ContactPointGroupingGene,
    },
};

pub struct ContactPointGroupingEvaluatorSettings<'a> {
    settings: &'a Settings,
    graph: &'a SurfaceGraph,
    rand: Random,
    evaluator: SupportStructureEvaluatorSettings<'a>,
    points: &'a ContactPointsGene,
}

impl<'a> ContactPointGroupingEvaluatorSettings<'a> {
    pub fn new(
        settings: &'a Settings,
        graph: &'a SurfaceGraph,
        rand: Random,
        evaluator: SupportStructureEvaluatorSettings<'a>,
        points: &'a ContactPointsGene,
    ) -> Self {
        Self {
            settings,
            graph,
            rand,
            evaluator,
            points,
        }
    }
}

pub struct ContactPointGroupingEvaluator<'a> {
    evaluator: FullEvaluator<'a>,
    settings: &'a Settings,
    rand: Random,
    points: &'a ContactPointsGene,
}

impl<'a> Evaluator<ContactPointGroupingGene, ContactPointGroupingEvaluatorSettings<'a>>
    for ContactPointGroupingEvaluator<'a>
{
    fn new(settings: &ContactPointGroupingEvaluatorSettings<'a>) -> Self {
        Self {
            evaluator: FullEvaluator::new(&settings.evaluator),
            settings: settings.settings,
            rand: settings.rand.seeded_copy(),
            points: settings.points,
        }
    }

    fn evaluate(&self, gene: &ContactPointGroupingGene) -> Cost {
        let s = &self.settings.contact_points_grouping_settings;
        let gene = gene.to_compressed_gene(self.points, self.evaluator.graph, self.settings, &self.rand);
        let size_cost: f32 = gene.groups.iter().map(|g| {
            let p = g.support_positions();
            let h = ConvexHull::new(p.collect());
            let area = h.area();
            let perimeter = h.perimeter();
            area * s.area_minimization_weight + perimeter * s.perimeter_minimization_weight
        }).sum();
        return Cost::new(size_cost)
    }

    fn visualize(&self, gene: &ContactPointGroupingGene) -> anyhow::Result<()> {
        let gene = gene.to_full_gene(self.points, self.evaluator.graph, self.settings, &self.rand);
        self.evaluator.visualize(&gene)
    }
}
