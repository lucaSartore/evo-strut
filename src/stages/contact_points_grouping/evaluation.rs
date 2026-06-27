use crate::stages::visualization::Color;
use crate::{
    evolution::{Cost, Evaluator},
    models::{Settings, SurfaceGraph},
    stages::{
        contact_point_optimization::ContactPointsGene,
        contact_points_grouping::models::ContactPointGroupingGene,
    },
    support::convex_hull_3d::ConvexHull3D,
};
use rerun::RecordingStream;

pub struct ContactPointGroupingEvaluatorSettings<'a> {
    settings: &'a Settings,
    graph: &'a SurfaceGraph,
    points: &'a ContactPointsGene,
}

impl<'a> ContactPointGroupingEvaluatorSettings<'a> {
    pub fn new(
        settings: &'a Settings,
        graph: &'a SurfaceGraph,
        points: &'a ContactPointsGene,
    ) -> Self {
        Self {
            settings,
            graph,
            points,
        }
    }
}

pub struct ContactPointGroupingEvaluator<'a> {
    settings: &'a Settings,
    points: &'a ContactPointsGene,
    stream: RecordingStream,
    graph: &'a SurfaceGraph,
}

impl<'a> Evaluator<ContactPointGroupingGene, ContactPointGroupingEvaluatorSettings<'a>>
    for ContactPointGroupingEvaluator<'a>
{
    fn new(settings: &ContactPointGroupingEvaluatorSettings<'a>) -> Self {
        Self {
            settings: settings.settings,
            points: settings.points,
            stream: rerun::RecordingStreamBuilder::new("grouped contact points")
                .spawn()
                .expect("failed to build rerun stream"),
            graph: settings.graph,
        }
    }

    fn evaluate(&self, gene: &ContactPointGroupingGene) -> Cost {
        let s = &self.settings.contact_points_grouping_settings;
        let groups = gene.to_groups(self.points, self.graph);
        let size_cost: f32 = groups
            .iter()
            .map(|g| {
                let p = g.contact_positions();
                let h = ConvexHull3D::new(p);
                let area = h.area();
                let volume = h.volume();
                let height = g.max_height();
                area * s.area_minimization_weight
                    + volume * s.volume_minimization_weight
                    + height * s.group_cost_penalty
            })
            .sum();

        Cost::new(size_cost)
    }

    fn visualize(&self, gene: &ContactPointGroupingGene) -> anyhow::Result<()> {
        let groups = gene.to_groups(self.points, self.graph);
        let graph = self.graph;

        self.stream.log(
            "mesh",
            &rerun::Mesh3D::new(graph.iter_vertices())
                .with_vertex_normals(graph.vertex_normals(None))
                .with_vertex_colors(vec![Color::Green; graph.count_vertices()])
                .with_triangle_indices(graph.iter_triangles(None)),
        )?;

        let num_groups = groups.len();
        let mut centers = Vec::new();
        let mut radiuses = Vec::new();
        let mut colors = Vec::new();
        let mut labels = Vec::new();

        for (group_id, group) in groups.iter().enumerate() {
            let hue = if num_groups == 0 {
                0.0
            } else {
                group_id as f32 * 360.0 / num_groups as f32
            };
            let color = Color::Hsv(hue, 1.0, 1.0);

            for contact in &group.contacts {
                centers.push(contact.position);
                radiuses.push(contact.radius);
                colors.push(color);
                labels.push(format!("group {}", group_id));
            }
        }

        self.stream.log(
            "contact_groups",
            &rerun::Cylinders3D::from_lengths_and_radii(vec![0.; centers.len()], radiuses)
                .with_centers(centers)
                .with_colors(colors)
                .with_labels(labels),
        )?;

        Ok(())
    }
}
