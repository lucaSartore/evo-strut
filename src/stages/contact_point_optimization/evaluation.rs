use std::collections::{HashMap, HashSet};
use crate::{evolution::{Cost, Evaluator}, models::{ Point, Settings, SurfaceGraph, TriangleId}, stages::contact_point_optimization::models::ContactPointsGene};
use itertools::Itertools;
use log::info;

mod bucketed_triangles;
use bucketed_triangles::BucketedTriangles;

mod surface_grid;
use smallvec::SmallVec;
use surface_grid::SurfaceGrid;


pub struct ContactPointEvaluatorSettings<'a> {
    pub graph: &'a SurfaceGraph,
    pub settings: &'a Settings,
    pub area: &'a [TriangleId],
    pub critical: &'a HashSet<TriangleId>
    
}
impl<'a> ContactPointEvaluatorSettings<'a> {
    pub fn new(
        graph: &'a SurfaceGraph,
        settings: &'a Settings,
        area: &'a [TriangleId],
        critical: &'a HashSet<TriangleId>
    ) -> Self {
        Self {
            graph, settings, area, critical
        }
    }
}

fn identifier_to_zero_point(discretization_size: f32, identifier: (i32, i32)) -> Point {
    let x = identifier.0 as f32 * discretization_size;
    let y = identifier.1 as f32 * discretization_size;
    Point{ x, y, z: 0. }
}
fn find_approximated_identifier(discretization_size: f32, point: Point) -> (i32, i32) {
    let x = (point.x / discretization_size).round() as i32;
    let y = (point.y / discretization_size).round() as i32;
    (x,y)
}

#[derive(Debug)]
pub struct SinglePointEvaluator {
    /// id of the element we are evaluating
    pub id: (i32, i32),
    /// cost of the current unit (dependent on his area, and his angle
    pub unit_cost: Cost,
    /// basic cost independent of all the neighbors (can be 0 if we have a lower
    /// non critical neighbors, a certain constant otherwise)
    pub base_cost: Cost,
    /// set of neighbors that we can inherit the cost from
    /// i.e. those are the critical neighbors that are below me
    pub critical_lower_neighbors: SmallVec<[(i32, i32); 4]>
}

impl SinglePointEvaluator {
    pub fn evaluate(&self, costs: &mut HashMap<(i32, i32), Cost>, supported: &HashSet<(i32, i32)>) -> Cost {
        
        if supported.contains(&self.id) {
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
        costs.insert(self.id, cost);
        
        cost
    }
}

pub struct ContactPointEvaluator<'a> {
    graph: &'a SurfaceGraph,
    settings: Settings,
    area_to_evaluate: &'a [TriangleId],
    critical: &'a HashSet<TriangleId>,
    surface_grid: SurfaceGrid,
    evaluation_order: Vec<SinglePointEvaluator>
}

impl<'a> ContactPointEvaluator<'a> {
    
    fn fill_evaluation_order(&mut self) {
        let mut eo: Vec<SinglePointEvaluator> = self.surface_grid
            .bucketed_triangles
            .iter_coordinates()
            .flat_map(|x| self.try_build_single_point_evaluator(*x))
            .collect();

        eo.sort_by_key(|x| {
            let point = identifier_to_zero_point(self.surface_grid.discretization_size, x.id);
            let triangle_id = self
                .surface_grid
                .bucketed_triangles
                .find_triangle_that_includes(self.graph, point)
                .expect("triangle should be always present");
            
            let triangle = self.graph.get_triangle(triangle_id);

            let z = triangle.find_z(point.x, point.y);
            // cost implement ord trait (f32 does not)
            Cost::new(z)
        });

        self.evaluation_order = eo;
    }

    fn try_build_single_point_evaluator(&self, id: (i32, i32)) -> Option<SinglePointEvaluator> {
        let point = self.surface_grid.points.get(&id)?;
        if !self.critical.contains(&point.id) {
            return None;
        }
        
        let has_lower_non_critical_neighbor = point.iter_neighbors(Some(true), Some(false)).count() != 0;
        let critical_lower_neighbors: SmallVec<[_;4]> = point
            .iter_neighbors(Some(true), Some(true))
            .map(|x| x.id)
            .collect();

        let cpos = &self.settings.contact_points_optimization_settings;
        // if 0 all is supported, if 90 nothing is supported
        let threshold = 90. - self.settings.criticality_settings.support_overhanging_angle;

        let difference = (threshold - point.angle).max(0.);
        let max_difference = (threshold - 0.0).max(0.);

        let angle_multiplier = difference / max_difference;

        let base_cost = if has_lower_non_critical_neighbor {0.0} else {cpos.cost_of_unsupported_min_point};
        let unit_cost = cpos.discretization_size.powi(2) * cpos.non_supported_cost * angle_multiplier;

        SinglePointEvaluator{
            id, 
            critical_lower_neighbors,
            base_cost: Cost::new(base_cost),
            unit_cost: Cost::new(unit_cost)
        }.into()
    }
}

impl<'a> Evaluator<ContactPointsGene, ContactPointEvaluatorSettings<'a>> for ContactPointEvaluator<'a> {
    fn new(settings: &ContactPointEvaluatorSettings<'a>) -> Self {
        let mut s = Self {
            graph: settings.graph,
            settings: settings.settings.clone(),
            area_to_evaluate: settings.area,
            critical: settings.critical,
            surface_grid: SurfaceGrid::new(
                settings.graph,
                settings.area,
                settings.critical,
                settings.settings.contact_points_optimization_settings.discretization_size
            ),
            evaluation_order: vec![]
        };
        s.fill_evaluation_order();
        s
    }

    fn evaluate(&self, gene: &ContactPointsGene) -> Cost {
        // todo: define the supported element from the gene
        let supported = HashSet::new();
        let mut costs = HashMap::new();
        let mut cost = Cost::ZERO;
        for e in self.evaluation_order.iter() {
            let new_cost = e.evaluate(&mut costs, &supported);
            cost = cost + new_cost;
        }
        cost
    }
}
