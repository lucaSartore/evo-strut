use std::collections::BinaryHeap;
use std::collections::HashSet;
use crate::models::Triangle;
use crate::{evolution::Cost, models::{ Point, Settings, SurfaceGraph, FaceId}};
use hashbrown::HashMap;
use itertools::Itertools;
use smallvec::SmallVec;



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct QueuedElement {
    pub id: FaceId,
    pub cost: Cost
}
#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for QueuedElement {
    // order is inverted in order to use the std "max-heap" (instead of haveing
    // to create a custom min-heap)
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
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

pub struct CostWithArea {
    pub unit_cost: Cost,
    pub area: f32
}
impl CostWithArea {
    pub fn absolute_cost(&self) -> Cost {
        self.unit_cost.times(self.area)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub base_cost: Cost,
    // area of the triangle evaluated
    pub area: f32
}


struct EvaluatedLayer {
    /// list of the triangles part of this layer
    triangles: HashMap<FaceId, EvaluatedTriangle>
}

impl EvaluatedLayer {
    pub fn new(
        graph: &SurfaceGraph,
        known_costs: &HashMap<FaceId, Cost>,
        current_layer: &[FaceId],
        in_below_layers: &HashSet<FaceId>,
        settings: &Settings
    ) -> Self {
        let mut e = Self {
            triangles: current_layer
                .iter()
                .filter(|x| !known_costs.contains_key(*x))
                .map(|x| (
                    *x,
                    EvaluatedTriangle{
                        base_cost: Cost::MAX,
                        id: *x,
                        same_layer_neighbors: Default::default(),
                        lower_layers_neighbors: Default::default(),
                        area: graph.get_triangle(*x).area()
                    }
                ))
                .collect()
        };
        e.fill_base_cost(graph, known_costs, settings);
        e.fill_same_layer_neighbors(graph, current_layer, settings);
        e.fill_lower_layers_neighbors(graph, in_below_layers, settings);
        e
    }

    /// calculate the cost surplus faced to move from two critical
    /// surfaces with center in p1 and p2)
    fn evaluate_cost_surplus(from: &Triangle<'_>, to: &Triangle<'_>, settings: &Settings) -> Cost {
        let from_center = from.center();
        let to_center= to.center();
        let distance = (from_center - to_center).abs();
        let propagation_factor = settings.contact_points_optimization_settings.cost_surplus_propagation_factor;

        let angle = Point::angle_between(&Point::DOWNWARD, &to.normal())
            .to_degrees()
            .clamp(0., 90.);

        // 0 => nothing is supported; 90 => everything is supported
        let angle_threshold = 90. - settings.criticality_settings.support_overhanging_angle;

        // is positive if cost should increase, negative if cost should decrease
        let angle_difference = angle_threshold - angle;

        let c = propagation_factor * distance * angle_difference;
        Cost::new(c)
    }
    // fn evaluate_cost_surplus(from: &Triangle<'_>, to: &Triangle<'_>, settings: &Settings) -> Cost {
    //     let from_center = from.center();
    //     let to_center= to.center();
    //     let distance = (from_center - to_center).abs();
    //     let propagation_factor = settings.contact_points_optimization_settings.cost_surplus_propagation_factor;
    //
    //     // if triangles are UPWARD facing id does not matter the inclination...
    //     // they are never critical.
    //     let angle_to = Point::angle_between(&Point::UPWARD, &to.normal()).to_degrees();
    //     let angle_from = Point::angle_between(&Point::UPWARD, &from.normal()).to_degrees();
    //     if angle_to < 90. || angle_from < 90. {
    //         return Cost::new(-propagation_factor * distance * 90.)
    //     }
    //
    //     let angle = if to_center.z <= from_center.z + f32::EPSILON {
    //         0.
    //     } else {
    //         Point::horizon_angle(from_center, to_center).to_degrees()
    //     }.clamp(0., 90.);
    //     // 0 => nothing is supported; 90 => everything is supported
    //     let angle_threshold = 90. - settings.criticality_settings.support_overhanging_angle;
    //
    //     // is positive if cost should increase, negative if cost should decrease
    //     let angle_difference = angle_threshold - angle;
    //
    //     let c = propagation_factor * distance * angle_difference;
    //     Cost::new(c)
    // }

    fn fill_base_cost(&mut self, graph: &SurfaceGraph, known_costs: &HashMap<FaceId, Cost>, settings: &Settings) {
        for (_,t) in self.triangles.iter_mut() {
            let this = graph.get_triangle(t.id);
            let this_layer = this.center().layer(settings);
            t.base_cost = *graph
                .iter_adjacent(this.index)
                .filter(|x| x.center().layer(settings) <= this_layer)
                .flat_map(|x| known_costs.get(&x.index))
                .min()
                .unwrap_or(&Cost::new(settings.contact_points_optimization_settings.non_supported_base_cost));
        }
    }

    fn fill_same_layer_neighbors(&mut self, graph: &SurfaceGraph, current_layer: &[FaceId], settings: &Settings) {
        let current_layer_set: HashSet<_> = current_layer.iter().collect();
        for (current_id, triangle) in self.triangles.iter_mut() {
            let current_triangle = graph.get_triangle(*current_id);
            graph.iter_adjacent(*current_id) 
                .filter(|adj| current_layer_set.contains(&adj.index))
                .for_each(|adj| {
                    let cost_surplus_forward = Self::evaluate_cost_surplus(&current_triangle, &adj, settings);
                    let cost_surplus_backward = Self::evaluate_cost_surplus(&adj, &current_triangle, settings);
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
            let current_triangle = graph.get_triangle(*current_id);
            graph.iter_adjacent(*current_id) 
                .filter(|adj| in_below_layers.contains(&adj.index))
                .for_each(|adj| {
                    let cost_surplus_forward = Self::evaluate_cost_surplus(&current_triangle, &adj, settings);
                    let cost_surplus_backward = Self::evaluate_cost_surplus(&adj, &current_triangle, settings);
                    let n = Neighbor{ 
                        cost_surplus_forward,
                        cost_surplus_backward,
                        id: adj.index
                    };
                    triangle.lower_layers_neighbors.push(n);
                });
        }
    }

    pub fn evaluate(&self, costs: &mut HashMap<FaceId, CostWithArea>, is_supported: &impl Fn(FaceId) -> bool) {
        let mut to_evaluate = self.triangles.len();
        let mut queue = BinaryHeap::new();
        let mut id_to_current_cost = HashMap::new();
        for t in self.triangles.values() {
            let base_cost = if is_supported(t.id) { Cost::ZERO } else { t.base_cost };
            let cost = t
                .lower_layers_neighbors
                .iter()
                .map(|x| costs[&x.id].unit_cost + x.cost_surplus_backward)
                .min()
                .unwrap_or(base_cost)
                .clamp(Cost::ZERO, base_cost);

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
            // _ = costs.insert(popped.id, popped.cost.times(self.triangles[&popped.id].area));
            let cwa = CostWithArea { unit_cost: popped.cost, area: self.triangles[&popped.id].area };
            _ = costs.insert(popped.id, cwa );

            // publishing recurrent cost for neighbor
            let triangle = self.triangles.get(&popped.id).expect("triangle should always be found");
            for n in &triangle.same_layer_neighbors {
                let neighbor_recursive_cost = (popped.cost + n.cost_surplus_forward).max(Cost::ZERO);
                let neighbor_current_cost = *id_to_current_cost.get(&n.id).unwrap_or(&Cost::MAX);
                if neighbor_recursive_cost < neighbor_current_cost {
                    _ = id_to_current_cost.insert(n.id, neighbor_recursive_cost);
                    queue.push(QueuedElement::new(n.id, neighbor_recursive_cost));
                }
            }
        }
    }
}


pub struct PropagationEvaluator<'a> {
    graph: &'a SurfaceGraph,
    settings: &'a Settings,
    area: &'a [FaceId],
    known_costs: &'a HashMap<FaceId, Cost>,
    layers: Vec<EvaluatedLayer>
}

impl<'a> PropagationEvaluator<'a> {

    pub fn new(
        graph: &'a SurfaceGraph,
        settings: &'a Settings,
        area: &'a [FaceId],
        known_costs: &'a HashMap<FaceId, Cost>
    ) -> Self {
        let mut to_return = Self {
            graph,
            settings,
            area,
            known_costs,
            layers: vec![]
        };
        to_return.fill_evaluation_layers();
        to_return
    }

    fn fill_evaluation_layers(&mut self) {
        let layers = self.area
            .iter()
            .filter(|x| !self.known_costs.contains_key(*x))
            .copied()
            .map(|x| {
                let p = self.graph.get_triangle(x).center();
                let layer = p.layer(self.settings);
                (layer, x)
            })
            .into_group_map();

        let mut in_below_layers = HashSet::new();

        for (_, layer) in layers.iter().sorted_by_key(|x| x.0) {
            let el = EvaluatedLayer::new(
                self.graph,
                self.known_costs,
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

    pub fn evaluate(&self, is_supported: &impl Fn(FaceId) -> bool) -> HashMap<FaceId, CostWithArea> {
        let mut costs = HashMap::new();
        for l in &self.layers {
            l.evaluate(&mut costs, is_supported);
        }
        costs
    }
} 
