use std::{any, collections::{HashMap, HashSet}};
use crate::{evolution::{Cost, Evaluator}, models::{ Point, Settings, SurfaceGraph, TriangleId}, stages::{contact_point_optimization::models::ContactPointsGene, visualization::Color}};
use itertools::Itertools;
use log::{debug, info};
use anyhow::{Result, anyhow};


mod surface_grid;
use surface_grid::*;

use smallvec::SmallVec;


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
    pub area_to_evaluate: &'a [TriangleId],
    pub critical: &'a HashSet<TriangleId>,
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
        let map = self.evaluate_internal(gene).0;
        Ok(())
    }
}
