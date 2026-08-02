use std::collections::VecDeque;

use crate::{
    evolution::Random,
    models::{MaterialStiffnessSettings, SupportStructureCostSettings},
    stages::support_structure_optimization::SupportStructureOptimizationGene,
};
use hashbrown::HashMap;
use nalgebra::DMatrix;
use smallvec::SmallVec;

use crate::{
    evolution::Cost,
    models::Point,
    stages::support_structure_optimization::{
        evaluation::{
            graph::StructureGraph,
            stiffness::{stiffness_parallel, stiffness_series, Stiffness},
        },
        SupportNodeId,
    },
};

use super::*;

pub struct DescriptorNodeDetails {
    pub id: SupportNodeId,
    pub position: Point,
    pub radius: f32,
    pub is_contact: bool,
}

#[derive(Default)]
pub struct GraphDescriptor {
    /// nodes with relative position ordered by height
    pub nodes: Vec<SupportNodeId>,
    pub details: HashMap<SupportNodeId, DescriptorNodeDetails>,
    pub edges: HashMap<SupportNodeId, SmallVec<[SupportNodeId; 4]>>,
}
impl GraphDescriptor {
    pub fn add_node(
        &mut self,
        id: SupportNodeId,
        position: Point,
        _supported: bool,
        is_contact: bool,
        radius: f32,
    ) {
        self.nodes.push(id);
        self.details.insert(
            id,
            DescriptorNodeDetails {
                id,
                position,
                radius,
                is_contact,
            },
        );
    }

    pub fn sort(&mut self) {
        self.nodes
            .sort_by_key(|x| Cost::new(self.details[x].position.z));
    }

    pub fn add_link(&mut self, from_id: SupportNodeId, to_id: SupportNodeId) {
        self.edges.entry(from_id).or_default().push(to_id);

        self.edges.entry(to_id).or_default().push(from_id);
    }

    pub fn new_random_id(&self, rand: &Random) -> SupportNodeId {
        let id = SupportNodeId(rand.next_u32());
        // re-generate it, as it is already taken
        if self.is_id_present(id) {
            return self.new_random_id(rand);
        }
        id
    }

    fn is_id_present(&self, id: SupportNodeId) -> bool {
        self.details.contains_key(&id)
    }
}

fn evaluate_steepness_cost(descriptor: &GraphDescriptor, settings: &Settings) -> Cost {
    let threshold = 90. - settings.criticality_settings.support_overhanging_angle;
    let distance_times_angle: f32 = descriptor
        .edges
        .iter()
        .map(|(node, neighbours)| {
            let node_position = descriptor.details[node].position;
            let len: f32 = neighbours
                .iter()
                .filter(|neighbour| **neighbour < *node)
                .map(|neighbour| {
                    let neighbour_position = descriptor.details[neighbour].position;
                    let angle = Point::horizon_angle(node_position, neighbour_position)
                        .to_degrees()
                        .abs();
                    let distance = (node_position - neighbour_position).abs();
                    let surplus = threshold - angle;
                    if surplus > 0. {
                        return surplus * distance;
                    }
                    0.0
                })
                .sum();
            len
        })
        .sum();
    Cost::new(
        distance_times_angle
            * settings
                .support_structure_cost_settings
                .cost_for_support_too_steep,
    )
}

fn evaluate_length_cost(descriptor: &GraphDescriptor, settings: &Settings) -> Cost {
    let surface: f32 = descriptor
        .edges
        .iter()
        .map(|(node, neighbours)| {
            let node_descriptor = &descriptor.details[node];
            let len: f32 = neighbours
                .iter()
                .filter(|neighbour| **neighbour < *node)
                .map(|neighbour| {
                    let neighbour_descriptor = &descriptor.details[neighbour];
                    let len = (node_descriptor.position - neighbour_descriptor.position).abs();
                    let radius = (node_descriptor.radius + neighbour_descriptor.radius) / 2.;
                    // approximated lateral surface o the cone
                    len * radius * 2.0 * std::f32::consts::PI
                })
                .sum();
            len
        })
        .sum();
    Cost::new(
        surface
            * settings
                .support_structure_cost_settings
                .cost_for_support_area,
    )
}

fn evaluate_single_support_stiffness(
    base_stiffness: &Stiffness,
    from: Point,
    to: Point,
    radius: f32,
    settings: &Settings,
) -> f32 {
    let s = &settings.support_structure_cost_settings;
    let to_integrate = Point::interpolate(from, to, s.stiffness_cost_integration_size);
    let vector = to - from;
    let mut cost = 0.;
    for i in 1..to_integrate.len() {
        let scalar = i as f32 / (to_integrate.len() - 1) as f32;
        let new_to = from + vector.to_scaled(scalar);
        let stiffness = stiffness_series(
            base_stiffness,
            from,
            new_to,
            radius,
            &s.material_stiffness_settings,
        );
        let c = compliance_value(s, &stiffness);
        cost += s.non_stiffness_cost * (c - s.non_stiffness_threshold).max(0.0);
    }
    cost
}

fn compliance_value(s: &SupportStructureCostSettings, stiffness: &Stiffness) -> f32 {
    let Some(compliance) = stiffness.0.try_inverse() else {
        // zero matrix mean node is somehow floating, we need
        // to add maximum cost
        if stiffness.0.iter().all(|x| *x == 0.) {
            return s.max_non_stiffness_cost / s.non_stiffness_cost;
        }
        panic!("fail to invert stiffness matrix: {:?}", stiffness)
    };
    let cxx = compliance[(0, 0)];
    let cyy = compliance[(1, 1)];
    (cxx.powi(2) + cyy.powi(2)).sqrt()
}

// recursively evaluate the stiffness of a graph
fn evaluate_stiffness<'b, 'a: 'b>(
    point: SupportNodeId,
    graph: &StructureGraph,
    cache: &'a mut HashMap<SupportNodeId, Stiffness>,
    settings: &MaterialStiffnessSettings,
) {
    if cache.contains_key(&point) {
        return;
    }
    let node = &graph.nodes[&point];

    if node.supported {
        cache.insert(point, Stiffness::SUPPORTED_STIFFNESS.clone());
        return;
    }
    let node_position = node.position;

    let supporters = graph.get_weighted_supports(point);

    let mut stiffness_to_combine = Vec::with_capacity(supporters.len());
    for (support, weight) in supporters {
        evaluate_stiffness(support, graph, cache, settings);
        let stiffness = &cache[&support];
        let weighted_stiffness = Stiffness(stiffness.0 * weight);
        let supporter = &graph.nodes[&support];
        stiffness_to_combine.push(stiffness_series(
            &weighted_stiffness,
            supporter.position,
            node_position,
            supporter.radius.min(node.radius),
            settings,
        ));
    }
    let final_stiffness = stiffness_parallel(&stiffness_to_combine);

    cache.insert(point, final_stiffness);
}

// evaluate the stiffness of a single node given a graph, and then reset the graph to leave no
// distances
fn evaluate_single_node_stiffness(
    s: &SupportStructureCostSettings,
    graph: &mut StructureGraph,
    to_evaluate: SupportNodeId,
) -> Stiffness {
    let visited_nodes = graph.mark_distances(to_evaluate);
    let mut cache = HashMap::default();
    evaluate_stiffness(
        to_evaluate,
        graph,
        &mut cache,
        &s.material_stiffness_settings,
    );
    let base_stiffness = cache
        .remove(&to_evaluate)
        .expect("node shall always be found");
    graph.reset_some_nodes(&visited_nodes);
    base_stiffness
}

#[allow(dead_code)]
// simulate the inversion of a matrix, to simulate how slow the algorithm would be
// if ran using the direct stiffness method, instead of the approximated version used in here
fn invert_stiffness_matrix(size: usize) {
    let mut data = vec![0.0; size * size];
    for i in 0..size {
        for j in 0..size {
            if i == j {
                data[i * size + j] = size as f64 * 2.0; // Strong diagonal
            } else {
                data[i * size + j] = 1.0;
            }
        }
    }

    let matrix = DMatrix::from_vec(size, size, data);
    let _ = matrix.try_inverse();
}


fn evaluate_stiffness_cost(
    surface: &SurfaceGraph,
    descriptor: &GraphDescriptor,
    settings: &Settings,
    floating_regions: &[FloatingRegion],
) -> Cost {
    let s = &settings.support_structure_cost_settings;
    let mut floating_regions_collector = FloatingRegionsCollector::new(floating_regions, surface);
    let mut cost = 0.;
    let mut graph = StructureGraph::new();
    for node in &descriptor.nodes {
        let node_descriptor = &descriptor.details[node];
        let node_position = node_descriptor.position;
        let node_radius = node_descriptor.radius;
        cost += floating_regions_collector.dump_costs(s, node_position.z, &graph, surface);
        let supporters: Vec<_> = descriptor.edges[node]
            .iter()
            .filter(|x| graph.nodes.contains_key(*x))
            .collect();

        // line used to test what the performance impact of using accurate stiffness evaluation
        // would be
        // if graph.nodes.len().is_multiple_of(50) || graph.nodes.len() == descriptor.nodes.len() {
        //     invert_stiffness_matrix(6 * graph.nodes.len());
        // }

        for supporter in supporters {
            let supported_descriptor = &descriptor.details[supporter];
            let supporter_position = supported_descriptor.position;
            let supporter_radius = supported_descriptor.radius;
            let base_stiffness = evaluate_single_node_stiffness(s, &mut graph, *supporter);
            cost += evaluate_single_support_stiffness(
                &base_stiffness,
                supporter_position,
                node_position,
                node_radius.min(supporter_radius),
                settings,
            );
        }
        graph.add_node(
            *node,
            node_position,
            &descriptor.edges[node],
            node_position.z == 0.,
            node_radius,
        );
    }
    cost += floating_regions_collector.dump_costs(s, f32::MAX, &graph, surface);
    Cost::new(cost)
}

fn evaluate_floating_regions_stiffness_cost(
    s: &SupportStructureCostSettings,
    mut graph: StructureGraph,
    surface: &SurfaceGraph,
    r: &FloatingRegion,
) -> f32 {
    let pos_to_node_id = graph.build_pos_to_node_id();
    let faces_positions: Vec<_> = r
        .faces()
        .iter()
        .map(|x| surface.get_triangle(*x).center())
        .collect();

    let center = faces_positions
        .iter()
        .fold(Point::ZERO, |acc, v| acc + *v)
        .to_scaled(0.1 / faces_positions.len() as f32);

    let neighbors: Vec<_> = faces_positions.iter().map(|x| pos_to_node_id[x]).collect();

    let id = graph.new_random_id(&Random::UnSeededRandom);

    graph.add_node(
        id,
        center,
        &neighbors,
        false,
        s.floating_regions_equivalent_beam_radius,
    );

    let stiffness = evaluate_single_node_stiffness(s, &mut graph, id);
    let compliance = compliance_value(s, &stiffness);

    (compliance - r.compliance_threshold).max(0.) * s.floating_region_cost_weight
}

fn evaluate_collision_cost(
    descriptor: &GraphDescriptor,
    volume: &Volume,
    settings: &Settings,
) -> Cost {
    let mut cost = 0.;
    let s = &settings.support_structure_cost_settings;
    let interval_distance = s.collision_check_intervals;
    let collision_cost = s.collision_penalization;
    for (node_id, neighbors) in descriptor.edges.iter() {
        let node = &descriptor.details[node_id];
        for neighbor_id in neighbors.iter().filter(|x| *x < node_id) {
            let neighbor = &descriptor.details[neighbor_id];
            let interpolation =
                Point::interpolate(node.position, neighbor.position, interval_distance);
            if interpolation.len() < 2 {
                continue;
            }
            let distance = (interpolation[0] - interpolation[1]).abs();
            for p in interpolation {
                if volume.is_inside(Into::<[f32; 3]>::into(p)) {
                    cost += distance * collision_cost;
                }
            }
        }
    }
    Cost::new(cost)
}

struct FloatingRegionsCollector<'a> {
    elements: VecDeque<(f32, &'a FloatingRegion)>,
}

impl<'a> FloatingRegionsCollector<'a> {
    pub fn dump_costs(
        &mut self,
        s: &SupportStructureCostSettings,
        until_height: f32,
        graph: &StructureGraph,
        surface: &SurfaceGraph,
    ) -> f32 {
        let mut c = 0.;
        while let Some(e) = self.elements.front()
            && e.0 < until_height
        {
            let (_, region) = self.elements.pop_front().expect("can't be None");
            c += evaluate_floating_regions_stiffness_cost(s, graph.clone(), surface, region);
        }
        c
    }

    pub fn new(regions: &'a [FloatingRegion], surface: &SurfaceGraph) -> Self {
        let mut elements = Vec::new();
        for r in regions {
            elements.push((r.max_height(surface), r));
        }
        elements.sort_by_key(|x| Cost::new(x.0));
        Self {
            elements: elements.into_iter().collect(),
        }
    }
}

pub fn evaluate_cost(
    gene: &SupportStructureOptimizationGene,
    surface: &SurfaceGraph,
    volume: &Volume,
    settings: &Settings,
    floating_regions: &[FloatingRegion],
) -> Cost {
    let descriptor = gene.to_graph_descriptor(surface, settings);
    let steepness_cost = evaluate_steepness_cost(&descriptor, settings);
    let length_cost = evaluate_length_cost(&descriptor, settings);
    let stiffness_cost = evaluate_stiffness_cost(surface, &descriptor, settings, floating_regions);
    let collision_cost = evaluate_collision_cost(&descriptor, volume, settings);

    steepness_cost + length_cost + stiffness_cost + collision_cost
}
