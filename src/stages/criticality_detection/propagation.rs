use crate::models::Triangle;
use crate::{
    evolution::Cost,
    models::{FaceId, Point, Settings, SurfaceGraph},
};
use hashbrown::HashMap;
use itertools::Itertools;
use smallvec::SmallVec;
use std::collections::BinaryHeap;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueuedElement<T>
where
    T: Clone + PartialEq + Eq,
{
    pub id: T,
    pub value: Cost,
}
#[allow(clippy::non_canonical_partial_ord_impl)]
impl<T> PartialOrd for QueuedElement<T>
where
    T: Clone + PartialEq + Eq,
{
    // order is inverted in order to use the std "max-heap" (instead of haveing
    // to create a custom min-heap)
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.value.partial_cmp(&self.value)
    }
}
impl<T> Ord for QueuedElement<T>
where
    T: Clone + PartialEq + Eq,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}
impl<T> QueuedElement<T>
where
    T: Clone + PartialEq + Eq,
{
    pub fn new_from_value(id: T, value: f32) -> Self {
        Self {
            id,
            value: Cost::new(value),
        }
    }
    pub fn new(id: T, cost: Cost) -> Self {
        Self { id, value: cost }
    }
}

pub struct CostWithArea {
    pub unit_cost: Cost,
    pub area: f32,
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
    cost_surplus_backward: Cost,
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
    pub area: f32,
}

struct EvaluatedLayer<'a> {
    /// list of the triangles part of this layer
    triangles: HashMap<FaceId, EvaluatedTriangle>,
    graph: &'a SurfaceGraph
}

impl<'a> EvaluatedLayer<'a> {
    pub fn new<T>(
        graph: &'a SurfaceGraph,
        known_costs: &T,
        current_layer: &[FaceId],
        in_below_layers: &HashSet<FaceId>,
        settings: &Settings,
    ) -> Self
    where
        T: KnownCosts,
    {
        let mut e = Self {
            graph,
            triangles: current_layer
                .iter()
                .filter(|x| known_costs.is_cost_unknown_or_non_zero(**x))
                .map(|x| {
                    (
                        *x,
                        EvaluatedTriangle {
                            base_cost: Cost::MAX,
                            id: *x,
                            same_layer_neighbors: Default::default(),
                            lower_layers_neighbors: Default::default(),
                            area: graph.get_triangle(*x).area(),
                        },
                    )
                })
                .collect(),
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
        let to_center = to.center();
        let distance = (from_center - to_center).abs();
        let propagation_factor = settings
            .contact_points_optimization_settings
            .cost_surplus_propagation_factor;

        let angle = Point::angle_between(&Point::DOWNWARD, &to.normal())
            .to_degrees()
            .clamp(0., 90.);

        // 0 => nothing is supported; 90 => everything is supported
        let angle_threshold = 90. - settings.criticality_settings.support_overhanging_angle;

        // is positive if cost should increase, negative if cost should decrease
        let angle_difference = (angle_threshold - angle).clamp(
            -settings
                .contact_points_optimization_settings
                .critical_angle_clipping_factor_down,
            settings
                .contact_points_optimization_settings
                .critical_angle_clipping_factor_up,
        );

        let c = propagation_factor * distance * angle_difference;
        Cost::new(c)
    }

    // fill the base cost of every triangle in a layer.
    // the cost is filled up as follow:
    //  - if any adjacent triangle have a known cost, then the cost will be that + the cost surplus
    //  - otherwise we will set it to the cost of an unsupported triangle
    fn fill_base_cost<T>(&mut self, graph: &SurfaceGraph, known_costs: &T, settings: &Settings)
    where
        T: KnownCosts,
    {
        for (_, t) in self.triangles.iter_mut() {
            let this = graph.get_triangle(t.id);
            let this_layer = this.center().layer(settings);
            t.base_cost = graph
                .iter_adjacent(this.index)
                .filter(|x| x.center().layer(settings) <= this_layer)
                .flat_map(|x| {
                    let known_cost = known_costs.cost_of(x.index)?;
                    let surplus_cost = Self::evaluate_cost_surplus(&this, &x, settings);
                    Some((known_cost + surplus_cost).max(Cost::ZERO))
                })
                .min()
                .unwrap_or(Cost::new(
                    settings
                        .contact_points_optimization_settings
                        .non_supported_base_cost,
                ));
        }
    }

    fn fill_same_layer_neighbors(
        &mut self,
        graph: &SurfaceGraph,
        current_layer: &[FaceId],
        settings: &Settings,
    ) {
        let current_layer_set: HashSet<_> = current_layer.iter().collect();
        for (current_id, triangle) in self.triangles.iter_mut() {
            let current_triangle = graph.get_triangle(*current_id);
            graph
                .iter_adjacent(*current_id)
                .filter(|adj| current_layer_set.contains(&adj.index))
                .for_each(|adj| {
                    let cost_surplus_forward =
                        Self::evaluate_cost_surplus(&current_triangle, &adj, settings);
                    let cost_surplus_backward =
                        Self::evaluate_cost_surplus(&adj, &current_triangle, settings);
                    let n = Neighbor {
                        cost_surplus_forward,
                        cost_surplus_backward,
                        id: adj.index,
                    };
                    triangle.same_layer_neighbors.push(n);
                });
        }
    }

    fn fill_lower_layers_neighbors(
        &mut self,
        graph: &SurfaceGraph,
        in_below_layers: &HashSet<FaceId>,
        settings: &Settings,
    ) {
        for (current_id, triangle) in self.triangles.iter_mut() {
            let current_triangle = graph.get_triangle(*current_id);
            graph
                .iter_adjacent(*current_id)
                .filter(|adj| in_below_layers.contains(&adj.index))
                .for_each(|adj| {
                    let cost_surplus_forward =
                        Self::evaluate_cost_surplus(&current_triangle, &adj, settings);
                    let cost_surplus_backward =
                        Self::evaluate_cost_surplus(&adj, &current_triangle, settings);
                    let n = Neighbor {
                        cost_surplus_forward,
                        cost_surplus_backward,
                        id: adj.index,
                    };
                    triangle.lower_layers_neighbors.push(n);
                });
        }
    }

    /// evaluate the nodes in one layer by propagating the cost factor nodes by node.
    /// the cost of a node N1 adjacent to node N2 will be:
    ///  - cost(N2) + surplus_cost(N2, N1) + soft_cost_propagation_factor * base_cost(N1) * area(N1)
    pub fn evaluate(
        &self,
        costs: &mut HashMap<FaceId, CostWithArea>,
        is_supported: &impl Fn(FaceId) -> bool,
        soft_cost_propagation_factor: f32,
        additional_cost: &impl Fn(Point) -> Cost
    ) {
        let mut to_evaluate = self.triangles.len();
        let mut queue = BinaryHeap::new();
        let mut id_to_current_cost = HashMap::new();
        for t in self.triangles.values() {
            let base_cost = if is_supported(t.id) {
                Cost::ZERO
            } else {
                t.base_cost
            };
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
            let popped = queue
                .pop()
                .expect("queue should never empty before to_evaluate is zero");
            // point has already being evaluated
            if costs.contains_key(&popped.id) {
                continue;
            }
            let point = self.graph.get_triangle(popped.id).center();
            // adding the point to the known costs
            to_evaluate -= 1;
            // _ = costs.insert(popped.id, popped.cost.times(self.triangles[&popped.id].area));
            let cwa = CostWithArea {
                unit_cost: popped.value + additional_cost(point),
                area: self.triangles[&popped.id].area,
            };
            _ = costs.insert(popped.id, cwa);

            // publishing recurrent cost for neighbor
            let triangle = self
                .triangles
                .get(&popped.id)
                .expect("triangle should always be found");
            for n in &triangle.same_layer_neighbors {
                let neighbor_recursive_cost =
                    (
                        popped.value +
                        n.cost_surplus_forward +
                        Cost::new(self.triangles[&n.id].base_cost.as_f32() * soft_cost_propagation_factor * triangle.area)
                    ).max(Cost::ZERO);
                    
                let neighbor_current_cost = *id_to_current_cost.get(&n.id).unwrap_or(&Cost::MAX);
                if neighbor_recursive_cost < neighbor_current_cost {
                    _ = id_to_current_cost.insert(n.id, neighbor_recursive_cost);
                    queue.push(QueuedElement::new(n.id, neighbor_recursive_cost));
                }
            }
        }
    }
}

pub trait KnownCosts {
    fn cost_of(&self, id: FaceId) -> Option<Cost>;
    fn is_cost_unknown_or_non_zero(&self, id: FaceId) -> bool {
        match self.cost_of(id) {
            None => true,
            Some(c) => c > Cost::ZERO
        }
    }
}

pub struct PropagationEvaluator<'a, T>
where
    T: KnownCosts,
{
    graph: &'a SurfaceGraph,
    settings: &'a Settings,
    pub area: &'a [FaceId],
    pub known_costs: T,
    layers: Vec<EvaluatedLayer<'a>>,
}

impl<'a, T> PropagationEvaluator<'a, T>
where
    T: KnownCosts,
{
    pub fn new(
        graph: &'a SurfaceGraph,
        settings: &'a Settings,
        area: &'a [FaceId],
        known_costs: T,
    ) -> Self {
        let mut to_return = Self {
            graph,
            settings,
            area,
            known_costs,
            layers: vec![],
        };
        to_return.fill_evaluation_layers();
        to_return
    }

    fn fill_evaluation_layers(&mut self) {
        let layers = self
            .area
            .iter()
            .filter(|x| self.known_costs.is_cost_unknown_or_non_zero(**x))
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
                &self.known_costs,
                layer.as_slice(),
                &in_below_layers,
                self.settings,
            );
            self.layers.push(el);

            layer.iter().for_each(|e| {
                in_below_layers.insert(*e);
            });
        }
    }

    pub fn evaluate(
        &self,
        is_supported: &impl Fn(FaceId) -> bool,
        soft_cost_propagation_factor: f32,
        additional_cost: &impl Fn(Point) -> Cost
    ) -> HashMap<FaceId, CostWithArea> {
        let mut costs = HashMap::new();
        for l in &self.layers {
            l.evaluate(&mut costs, is_supported, soft_cost_propagation_factor, additional_cost);
        }
        costs
    }
}
