use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use crate::models::ContactPointsOptimizationSettings;
use crate::models::MeshId;
use crate::models::MeshVector;
use crate::stages::criticality_grouping;
use crate::{evolution::{Cost, Evaluator}, models::{ Point, Settings, SurfaceGraph, FaceId}, stages::{contact_point_optimization::models::ContactPointsGene, visualization::Color}};
use baby_shark::geometry::primitives::triangle2;
use itertools::Itertools;
use log::debug;
use anyhow::{Result, anyhow};
use nalgebra::distance;
use rerun::external::glam::usize;
use smallvec::SmallVec;
use smallvec::smallvec;



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct QueuedElement {
    pub id: FaceId,
    pub cost: Cost
}
impl PartialOrd for QueuedElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}
impl Ord for QueuedElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}
impl QueuedElement {
    pub fn new(id: FaceId, cost: Cost) -> Self {
        Self {
            id, cost
        }
    }
}

struct Neighbor {
    /// id of the neighbor
    id: FaceId,
    /// cost added to navigate from myself to the neighbor
    cost_surplus_forward: Cost,
    /// cost added to navigate from the neighbor to myself
    cost_surplus_backward: Cost
}

struct EvaluatedTriangle {
    /// list of neighbors that are part of the layer below.
    /// when evaluating we can be sure their criticality level
    /// is known
    pub lower_layers_neighbors: SmallVec<[Neighbor; 3]>,
    /// neighbors from the same layer as the current triangle.
    /// when evaluating them we can't be sure if their criticality
    /// level is known or not
    pub same_layer_neighbors: SmallVec<[Neighbor; 3]>,
    pub id: FaceId,
    /// max cost that will be used as self's cost if none
    /// of the neighbor have a low enough cost.
    /// can be an high constant if the surface is not supported,
    /// or a small value if the surface has a non critical neighbor
    pub base_cost: Cost
}


struct EvaluatedLayer {
    /// list of the triangles part of this layer
    triangles: HashMap<FaceId, EvaluatedTriangle>
}

impl EvaluatedLayer {
    pub fn new(
        graph: &SurfaceGraph,
        critical: &MeshVector<FaceId, bool>,
        current_layer: &[FaceId],
        in_below_layers: &HashSet<FaceId>,
        settings: &Settings
    ) -> Self {
        let mut e = Self {
            triangles: current_layer
                .iter()
                .filter(|x| critical[**x])
                .map(|x| (
                    *x,
                    EvaluatedTriangle{
                        base_cost: Cost::MAX,
                        id: *x,
                        same_layer_neighbors: Default::default(),
                        lower_layers_neighbors: Default::default()
                    }
                ))
                .collect()
        };
        e.fill_base_cost(graph, critical, settings);
        e.fill_same_layer_neighbors(graph, current_layer, settings);
        e.fill_lower_layers_neighbors(graph, in_below_layers, settings);
        e
    }

    /// calculate the cost surplus faced to move from two critical
    /// surfaces with center in p1 and p2)
    fn evaluate_cost_surplus(from: Point, to: Point, settings: &Settings) -> Cost {
        let distance = (from - to).abs();
        let angle = if to.z < from.z {
            90.
        } else {
            Point::horizon_angle(from, to).to_degrees()
        }.clamp(0., 90.);
        let propagation_factor = settings.contact_points_optimization_settings.cost_surplus_propagation_factor;

        let c = propagation_factor * distance * (90. - angle);
        Cost::new(c)
    }

    fn fill_base_cost(&mut self, graph: &SurfaceGraph, critical: &MeshVector<FaceId, bool>, settings: &Settings) {
        for (_,t) in self.triangles.iter_mut() {
            let center_this = graph.get_triangle(t.id).center();
            t.base_cost = graph
                .iter_adjacent(t.id)
                .filter(|adj| !critical[adj.index])
                .filter(|adj| adj.center().z < center_this.z)
                .map(|adj| {
                    let center_neighbor = adj.center();
                    Self::evaluate_cost_surplus(center_this, center_neighbor, settings)
                })
                .min()
                .unwrap_or(Cost::new(settings.contact_points_optimization_settings.non_supported_base_cost));
        }
    }

    fn fill_same_layer_neighbors(&mut self, graph: &SurfaceGraph, current_layer: &[FaceId], settings: &Settings) {
        let current_layer_set: HashSet<_> = current_layer.iter().collect();
        for (current_id, triangle) in self.triangles.iter_mut() {
            let current_center = graph.get_triangle(*current_id).center();
            graph.iter_adjacent(*current_id) 
                .filter(|adj| current_layer_set.contains(&adj.index))
                .for_each(|adj| {
                    let adj_center = adj.center();
                    let cost_surplus_forward = Self::evaluate_cost_surplus(current_center, adj_center, settings);
                    let cost_surplus_backward = Self::evaluate_cost_surplus(adj_center, current_center, settings);
                    let n = Neighbor{ 
                        cost_surplus_forward,
                        cost_surplus_backward,
                        id: adj.index
                    };
                    triangle.same_layer_neighbors.push(n);
                });
        }
    }

    fn fill_lower_layers_neighbors(&mut self, graph: &SurfaceGraph, in_below_layers: &HashSet<FaceId>, settings: &Settings) {
        for (current_id, triangle) in self.triangles.iter_mut() {
            let current_center = graph.get_triangle(*current_id).center();
            graph.iter_adjacent(*current_id) 
                .filter(|adj| in_below_layers.contains(&adj.index))
                .for_each(|adj| {
                    let adj_center = adj.center();
                    let cost_surplus_forward = Self::evaluate_cost_surplus(current_center, adj_center, settings);
                    let cost_surplus_backward = Self::evaluate_cost_surplus(adj_center, current_center, settings);
                    let n = Neighbor{ 
                        cost_surplus_forward,
                        cost_surplus_backward,
                        id: adj.index
                    };
                    triangle.lower_layers_neighbors.push(n);
                });
        }
    }

    pub fn evaluate(&self, costs: &mut HashMap<FaceId, Cost>) {
        let mut to_evaluate = self.triangles.len();
        let mut queue = BinaryHeap::new();
        let mut id_to_current_cost = HashMap::new();
        for t in self.triangles.values() {
            let cost = t
                .lower_layers_neighbors
                .iter()
                .map(|x| costs[&x.id] + x.cost_surplus_backward)
                .min()
                .unwrap_or(t.base_cost)
                .min(t.base_cost);

            id_to_current_cost.insert(t.id, cost);
            queue.push(QueuedElement::new(t.id, cost));
        }

        while to_evaluate != 0 {
            let popped = queue.pop().expect("queue should never empty before to_evaluate is zero");
            // point has already being evaluated
            if costs.contains_key(&popped.id) {
                continue
            }
            // adding the point to the known costs
            to_evaluate -= 1;
            _ = costs.insert(popped.id, popped.cost);

            // publishing recurrent cost for neighbor
            let triangle = self.triangles.get(&popped.id).expect("triangle should always be found");
            for n in &triangle.same_layer_neighbors {
                let neighbor_recursive_cost = popped.cost + n.cost_surplus_forward;
                let neighbor_current_cost = *id_to_current_cost.get(&n.id).unwrap_or(&Cost::MAX);
                if neighbor_recursive_cost < neighbor_current_cost {
                    _ = id_to_current_cost.insert(n.id, neighbor_recursive_cost);
                    queue.push(QueuedElement::new(n.id, neighbor_recursive_cost));
                }
            }
        }
    }
}

pub struct ContactPointEvaluatorSettings<'a> {
    pub graph: &'a SurfaceGraph,
    pub settings: &'a Settings,
    pub area: &'a [FaceId],
    pub critical: &'a MeshVector<FaceId, bool>
}
impl<'a> ContactPointEvaluatorSettings<'a> {
    pub fn new(
        graph: &'a SurfaceGraph,
        settings: &'a Settings,
        area: &'a [FaceId],
        critical: &'a MeshVector<FaceId, bool>
    ) -> Self {
        Self {
            graph, settings, area, critical
        }
    }

    
}


pub struct ContactPointEvaluator<'a> {
    graph: &'a SurfaceGraph,
    settings: &'a Settings,
    area: &'a [FaceId],
    critical: &'a MeshVector<FaceId, bool>,
    layers: Vec<EvaluatedLayer>
}

impl<'a> ContactPointEvaluator<'a> {
    fn fill_evaluation_layers(&mut self) {
        let layer_height = self.settings.contact_points_optimization_settings.layer_height;
        let layers = self.area
            .iter()
            .filter(|x| self.critical[**x])
            .copied()
            .map(|x| {
                let p = self.graph.get_triangle(x).center();
                let layer = (p.z / layer_height).ceil() as usize;
                (layer, x)
            })
            .into_group_map();

        let mut in_below_layers = HashSet::new();

        for (_,layer) in layers {
            let el = EvaluatedLayer::new(
                self.graph,
                self.critical,
                layer.as_slice(),
                &in_below_layers,
                self.settings
            );
            self.layers.push(el);

            layer.iter().for_each(|e| {
                in_below_layers.insert(*e);
            });
        }
    }
}

impl<'a> Evaluator<ContactPointsGene, ContactPointEvaluatorSettings<'a>> for ContactPointEvaluator<'a> {
    fn new(settings: &ContactPointEvaluatorSettings<'a>) -> Self {
        let mut e = Self {
            graph: settings.graph,
            settings: settings.settings,
            area: settings.area,
            critical: settings.critical,
            layers: vec![]
        };
        e.fill_evaluation_layers();
        e
    }

    fn evaluate(&self, gene: &ContactPointsGene) -> Cost {
        let mut costs = HashMap::new();
        for l in &self.layers {
            l.evaluate(&mut costs);
        }
        costs.iter().fold(Cost::ZERO, |acc, e| acc + *e.1)
    }
    
    fn visualize(&self, gene: &ContactPointsGene) -> Result<()> {
        // todo!();
        Ok(())
    }
}
