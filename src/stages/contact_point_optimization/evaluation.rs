use std::collections::HashSet;
use crate::models::MeshId;
use crate::models::MeshVector;
use crate::{evolution::{Cost, Evaluator}, models::{ Point, Settings, SurfaceGraph, FaceId}, stages::{contact_point_optimization::models::ContactPointsGene, visualization::Color}};
use hashbrown::HashMap;
use anyhow::{Result, anyhow};
use rerun::RecordingStream;
use rerun::external::glam::usize;
use crate::stages::criticality_detection::propagation::*;


pub struct ContactPointEvaluatorSettings<'a> {
    pub graph: &'a SurfaceGraph,
    pub settings: &'a Settings,
    pub area: &'a [FaceId],
    pub critical: &'a MeshVector<FaceId, bool>,
    pub area_index: usize
}
impl<'a> ContactPointEvaluatorSettings<'a> {
    pub fn new(
        graph: &'a SurfaceGraph,
        settings: &'a Settings,
        area: &'a [FaceId],
        critical: &'a MeshVector<FaceId, bool>,
        area_index: usize
    ) -> Self {
        Self {
            graph, settings, area, critical, area_index
        }
    }

    
}

pub struct CriticalBasedKnownCosts<'a> {
    critical: &'a MeshVector<FaceId, bool>
}
impl<'a> KnownCosts for CriticalBasedKnownCosts<'a> {
    fn cost_of(&self, id: FaceId) -> Option<Cost> {
        if self.critical[id] {
            return None
        } else {
            return Some(Cost::ZERO)
        }
    }
}

pub struct ContactPointEvaluator<'a> {
    graph: &'a SurfaceGraph,
    propagator: PropagationEvaluator<'a, CriticalBasedKnownCosts<'a>>,
    settings: &'a Settings,
    stream: RecordingStream,
}

impl<'a> ContactPointEvaluator<'a> {

    fn visualize(&self, costs: HashMap<FaceId, CostWithArea>, gene: &ContactPointsGene) -> Result<()> {

        let min = costs
            .values()
            .map(|x| x.unit_cost)
            .min()
            .ok_or(anyhow!("visualization_error: cost vector is empty"))?
            .as_f32();

        let max = costs
            .values()
            .map(|x| x.unit_cost)
            .max()
            .ok_or(anyhow!("visualization_error: cost vector is empty"))?
            .as_f32();

        let to_visualize_set: HashSet<_> = self.propagator.area.iter().copied().collect();

        let mut colors = vec![Color::Green; self.graph.count_vertices()];

        let normals = self.graph.vertex_normals(Some(&to_visualize_set));
        let triangles: Vec<_> = self.graph.iter_triangles(Some(&to_visualize_set)).collect();

        let mut cost_points = vec![];
        let mut cost_points_labels = vec![];

        // add colors
        for triangle in &triangles {

            // only critical triangles are colored
            if !self.propagator.known_costs.cost_of(triangle.index).is_none() {
                continue;
            }

            let points = triangle.vertexes();
            let indexes = triangle.vertexes_index();

            let cost = costs.get(&triangle.index).expect("triangle should always be found").unit_cost;

            cost_points.push(triangle.center());
            cost_points_labels.push(format!("{}", cost));

            for (_,i) in points.iter().zip(indexes.iter()) {

                // 1 if max cost, 0 otherwise
                let normalized = (cost.as_f32() - min) / (max + min);
                let normalized_u8 = (normalized * 255.0) as u8;

                colors[i.index()] = Color::Rgb(normalized_u8, 255 - normalized_u8, 0);
            }
        }

        let avg = self.graph.iter_triangles(Some(&to_visualize_set)).fold(
            Point{x: 0., y:0., z: 0.},
            |a,b| a+b.center()
        ).to_scaled(1.0 / to_visualize_set.len() as f32);

        let contact_points = gene
            .iter_contacts()
            .map(|p| self.graph.get_triangle(*p).center() - avg);

        let points = self
            .graph
            .iter_vertices()
            .map(|x| x - avg);

        cost_points.iter_mut().for_each(|x| *x = *x - avg);

        self.stream.log(
            "critical_mesh",
            &rerun::Mesh3D::new(points)
                .with_vertex_normals(normals)
                .with_vertex_colors(colors)
                .with_triangle_indices(triangles),
        )?;

        self.stream.log(
            "contact_points",
            &rerun::Points3D::new(contact_points)
        )?;

        let len = cost_points.len();
        self.stream.log(
            "triangle_costs",
            &rerun::Points3D::new(cost_points)
                .with_labels(cost_points_labels)
                .with_colors(vec![Color::Red; len])
        )?;

        Ok(())
    }

    fn evaluate_criticality_costs(&self, gene: &ContactPointsGene) -> HashMap<FaceId, CostWithArea> {
        self.propagator.evaluate(&|x| gene.is_supported(x))
   }

    pub fn evaluate_support_costs(&self, gene: &ContactPointsGene) -> Cost {
        let support_costs = gene.num_contacts() as f32 * self.settings.contact_points_optimization_settings.support_point_cost;
        let links_costs = gene
            .iter_links()
            .map(|(x,y)| {
                let cx = self.graph.get_triangle(x).center();
                let cy = self.graph.get_triangle(y).center();
                let d = (cx - cy).abs();
                self.settings.contact_points_optimization_settings.support_line_cost * d
            })
            .fold(0., |acc, x| acc + x);
        Cost::new(support_costs + links_costs)
    }
} 

impl<'a> Evaluator<ContactPointsGene, ContactPointEvaluatorSettings<'a>> for ContactPointEvaluator<'a> {
    fn new(settings: &ContactPointEvaluatorSettings<'a>) -> Self {
        let stream_name = format!("support of critical area {}", settings.area_index);
        let stream = rerun::RecordingStreamBuilder::new(stream_name)
            .spawn()
            .expect("failed to build stream");
        Self {
            graph: settings.graph,
            settings: settings.settings,
            stream,
            propagator: PropagationEvaluator::new(
                settings.graph,
                settings.settings,
                settings.area,
                CriticalBasedKnownCosts { critical: settings.critical}
            )
        }
    }

    fn evaluate(&self, gene: &ContactPointsGene) -> Cost {
        let costs = self.evaluate_criticality_costs(gene);
        let criticality_costs = costs.iter().fold(Cost::ZERO, |acc, e| acc + e.1.absolute_cost());
        let support_costs = self.evaluate_support_costs(gene);
        criticality_costs + support_costs
    }
    
    fn visualize(&self, gene: &ContactPointsGene) -> Result<()> {
        let costs = self.evaluate_criticality_costs(gene);
        self.visualize(costs, gene)
    }
}
