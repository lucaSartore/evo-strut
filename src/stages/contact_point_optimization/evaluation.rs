use std::{any, collections::{HashMap, HashSet}};
use crate::{evolution::{Cost, Evaluator}, models::{ Point, Settings, SurfaceGraph, TriangleId}, stages::{contact_point_optimization::models::ContactPointsGene, visualization::Color}};
use itertools::Itertools;
use log::{debug, info};
use anyhow::{Result, anyhow};


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
    // surface_grid: SurfaceGrid,
    evaluation_order: Vec<SinglePointEvaluator>
}

impl<'a> ContactPointEvaluator<'a> {
    
}

impl<'a> Evaluator<ContactPointsGene, ContactPointEvaluatorSettings<'a>> for ContactPointEvaluator<'a> {
    fn new(settings: &ContactPointEvaluatorSettings<'a>) -> Self {
        debug!(
            "area has {} elements, of which {} are critical",
            settings.area.len(),
            settings.area.iter().filter(|x| settings.critical.contains(x)).count(),
        );

        todo!();
        // let mut s = Self {
        //     graph: settings.graph,
        //     settings: settings.settings.clone(),
        //     area_to_evaluate: settings.area,
        //     critical: settings.critical,
        //     surface_grid: SurfaceGrid::new(
        //         settings.graph,
        //         settings.area,
        //         settings.critical,
        //         settings.settings.contact_points_optimization_settings.discretization_size
        //     ),
        //     evaluation_order: vec![]
        // };
        // s.fill_evaluation_order();
        // s
    }

    fn evaluate(&self, gene: &ContactPointsGene) -> Cost {
        todo!();
    }
    
    fn visualize(&self, gene: &ContactPointsGene) -> Result<()> {
        todo!();
    }
}
