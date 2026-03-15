use std::{any, collections::{HashMap, HashSet}};
use crate::{evolution::{Cost, Evaluator}, models::{ Point, Settings, SurfaceGraph, FaceId}, stages::{contact_point_optimization::models::ContactPointsGene, visualization::Color}};
use itertools::Itertools;
use log::{debug, info};
use anyhow::{Result, anyhow};


mod surface_grid;
use rerun::coordinates;
use surface_grid::*;

use smallvec::SmallVec;


pub struct ContactPointEvaluatorSettings<'a> {
    pub graph: &'a SurfaceGraph,
    pub settings: &'a Settings,
    pub area: &'a [FaceId],
    pub critical: &'a HashSet<FaceId>
    
}
impl<'a> ContactPointEvaluatorSettings<'a> {
    pub fn new(
        graph: &'a SurfaceGraph,
        settings: &'a Settings,
        area: &'a [FaceId],
        critical: &'a HashSet<FaceId>
    ) -> Self {
        Self {
            graph, settings, area, critical
        }
    }
}

#[derive(Debug)]
pub struct SinglePointEvaluator {
    /// id of the element we are evaluating
    pub coordinates: Coordinates,
    /// cost of the current unit (dependent on his area, and his angle
    pub unit_cost: Cost,
    /// basic cost independent of all the neighbors (can be 0 if we have a lower
    /// non critical neighbors, a certain constant otherwise)
    pub base_cost: Cost,
    /// set of neighbors that we can inherit the cost from
    /// i.e. those are the critical neighbors that are below me
    pub critical_lower_neighbors: SmallVec<[Coordinates; 4]>
}

impl SinglePointEvaluator {
    pub fn new(coordinates: Coordinates, evaluator: &ContactPointEvaluator<'_>) -> Self {
        let s = &evaluator.settings.contact_points_optimization_settings;

        let this = &evaluator.surface_grid.points[&coordinates];

        let has_lower_non_critical_neighbor = this
            .neighbors
            .iter()
            .any(|n_id| {
                let n = &evaluator.surface_grid.points[n_id];
                !n.critical && n.point.z < this.point.z
            });

        let base_cost = if has_lower_non_critical_neighbor { 0. } else { s.non_supported_base_cost };
        let unit_cost = s.non_supported_unit_cost * s.discretization_size.powi(2);
        let critical_lower_neighbors = this
            .neighbors
            .iter()
            .copied()
            .filter(|n_id| {
                let n = &evaluator.surface_grid.points[n_id];
                n.critical && n.point.z < this.point.z
            }).collect();
        
        Self {
            coordinates,
            unit_cost: Cost::new(unit_cost),
            base_cost: Cost::new(base_cost),
            critical_lower_neighbors
        }
    }

    pub fn evaluate(&self, costs: &mut HashMap<Coordinates, Cost>, supported: &HashSet<Coordinates>) -> Cost {
        
        if supported.contains(&self.coordinates) {
            return Cost::ZERO
        }

        let base_cost = self.critical_lower_neighbors
            .iter()
            .map(|x| costs[x])
            .min();
        
        let base_cost = match base_cost {
            Some(c) => c.min(self.base_cost),
            None => self.base_cost
        };

        let cost = base_cost + self.unit_cost;
        costs.insert(self.coordinates, cost);
        
        cost
    }
}

pub struct ContactPointEvaluator<'a> {
    pub graph: &'a SurfaceGraph,
    pub settings: Settings,
    pub area_to_evaluate: &'a [FaceId],
    pub critical: &'a HashSet<FaceId>,
    pub surface_grid: SurfaceGrid,
    pub evaluation_order: Vec<SinglePointEvaluator>
}

impl<'a> ContactPointEvaluator<'a> {

    fn fill_evaluation_order(&mut self) {
        self.evaluation_order = self
            .surface_grid
            .points
            .iter()
            .sorted_by_key(|x| Cost::new(x.1.point.z))
            .map(|x| SinglePointEvaluator::new(*x.0, self))
            .collect()
    }
    

    fn evaluate_internal(&self, gene: &ContactPointsGene) -> (HashMap<Coordinates, Cost>, Cost) {
        let mut costs = HashMap::new();

        // todo: fill this up
        let mut supported = HashSet::new();

        let mut cost = Cost::ZERO;
        for e in self.evaluation_order.iter() {
            cost = cost + e.evaluate(&mut costs, &mut supported);
        }
        (costs, cost)
    }

    fn visualize(&self, costs: HashMap<(i32, i32), Cost>) -> Result<()> {

        let min = costs
            .values()
            .min()
            .ok_or(anyhow!("visualization_error: cost vector is empty"))?
            .as_f32();

        let max = costs
            .values()
            .max()
            .ok_or(anyhow!("visualization_error: cost vector is empty"))?
            .as_f32();

        let to_visualize_set: HashSet<_> = self.area_to_evaluate.iter().copied().collect();

        let rec = rerun::RecordingStreamBuilder::new("critical_mesh").spawn()?;

        let mut colors = vec![Color::Green; self.graph.count_vertices()];

        let normals = self.graph.vertex_normals(Some(&to_visualize_set));
        let triangles: Vec<_> = self.graph.iter_triangles(Some(&to_visualize_set)).collect();


        // add colors
        for triangle in &triangles {

            // only critical triangles are colored
            if !self.critical.contains(&triangle.index) {
                continue;
            }

            let points = triangle.vertexes();
            let indexes = triangle.vertexes_index();
            for (p,i) in points.iter().zip(indexes.iter()) {
                let coordinates = self.surface_grid.point_to_discretized(*p);
                let cost = *costs.get(&coordinates).expect("triangle should always be found");

                // 1 if max cost, 0 otherwise
                let normalized = (cost.as_f32() - min) / (max + min);
                let normalized_u8 = (normalized * 255.0) as u8;

                colors[i.0 as usize] = Color::Rgb(normalized_u8, 255 - normalized_u8, 0);
            }
        }

        let avg = self.graph.iter_triangles(Some(&to_visualize_set)).fold(
            Point{x: 0., y:0., z: 0.},
            |a,b| a+b.center()
        ).to_scaled(1.0 / to_visualize_set.len() as f32);


        let points = self
            .graph
            .iter_vertices()
            .map(|x| x - avg);


        rec.log(
            "critical_mesh",
            &rerun::Mesh3D::new(points)
                .with_vertex_normals(normals)
                .with_vertex_colors(colors)
                .with_triangle_indices(triangles),
        )?;

        Ok(())
    }
}

impl<'a> Evaluator<ContactPointsGene, ContactPointEvaluatorSettings<'a>> for ContactPointEvaluator<'a> {
    fn new(settings: &ContactPointEvaluatorSettings<'a>) -> Self {
        debug!(
            "area has {} elements, of which {} are critical",
            settings.area.len(),
            settings.area.iter().filter(|x| settings.critical.contains(x)).count(),
        );

        let mut s = Self {
            graph: settings.graph,
            settings: settings.settings.clone(),
            area_to_evaluate: settings.area,
            critical: settings.critical,
            surface_grid: SurfaceGrid::new(
                settings.graph,
                settings.critical,
                settings.area,
                settings.settings.contact_points_optimization_settings.discretization_size
            ),
            evaluation_order: vec![]
        };
        s.fill_evaluation_order();
        s
    }

    fn evaluate(&self, gene: &ContactPointsGene) -> Cost {
        self.evaluate_internal(gene).1
    }
    
    fn visualize(&self, gene: &ContactPointsGene) -> Result<()> {
        let costs = self.evaluate_internal(gene).0;
        self.visualize(costs)
    }
}
