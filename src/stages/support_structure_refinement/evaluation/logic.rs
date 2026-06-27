use std::collections::VecDeque;

use crate::{
    evolution::Random,
    models::{MaterialStiffnessSettings, SupportStructureRefinementSettings},
};
use hashbrown::HashMap;
use smallvec::SmallVec;

use crate::{
    evolution::Cost,
    models::Point,
    stages::support_structure_refinement::{
        evaluation::{
            graph::Graph,
            stiffness::{stiffness_parallel, stiffness_series, Stiffness},
        },
        SupportNode, SupportNodeId,
    },
};

use super::*;

pub struct DescriptorNodeDetails {
    pub id: SupportNodeId,
    pub position: Point,
    pub radius: f32,
}
pub struct GraphDescriptor {
    /// nodes with relative position ordered by height
    pub nodes: Vec<SupportNodeId>,
    pub details: HashMap<SupportNodeId, DescriptorNodeDetails>,
    pub edges: HashMap<SupportNodeId, SmallVec<[SupportNodeId; 4]>>,
}

pub fn genome_to_graph_descriptor(gene: &SupportStructureGene) -> GraphDescriptor {
    let mut nodes = vec![];
    let mut details = HashMap::new();
    let mut edges = HashMap::new();
    for (_, g) in gene.nodes.iter() {
        let (node_id, position, adj) = match g {
            SupportNode::Contact(n) => (n.id, n.position, &n.leans_on),
            SupportNode::Base(n) => (n.id, n.last_position, &vec![]),
            SupportNode::Middle(n) => (n.id, n.last_position, &n.leans_on),
        };
        let radius = g.radius();
        nodes.push(node_id);
        details.insert(
            node_id,
            DescriptorNodeDetails {
                id: node_id,
                position,
                radius,
            },
        );
        for n in adj {
            edges.entry(*n).or_insert(vec![]).push(node_id);
        }
        edges
            .entry(node_id)
            .or_insert(vec![])
            .append(&mut adj.clone());
    }

    nodes.sort_by_key(|x| Cost::new(details[x].position.z));

    GraphDescriptor {
        nodes,
        details,
        edges: edges
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect(),
    }
}

fn cost_of_single_cone(
    base: Point,
    circle_center: Point,
    circle_radius: f32,
    settings: &Settings,
) -> f32 {
    let s = &settings.support_structure_refinement_settings;
    if base.z >= circle_center.z {
        return s.cost_of_un_feasible_cone;
    }
    let vec = circle_center - base;
    let height = vec.z.abs();

    let base_offset = (vec.x.powi(2) + vec.y.powi(2)).sqrt();

    //approximated area of the cone
    let side = ((base_offset + circle_radius).powi(2) + height.powi(2)).sqrt();
    let area = side * circle_radius * std::f32::consts::PI;

    let steepness = f32::atan(height / (circle_radius + base_offset)).to_degrees();
    let threshold = 90. - settings.criticality_settings.support_overhanging_angle;

    let mut cost = 0.;
    // steepness cost
    if steepness < threshold {
        cost += (threshold - steepness) * s.cone_too_steep_cost * side;
    }
    // area cost
    cost += area * s.cone_area_cost;

    cost
}

fn evaluate_cones_cost(
    gene: &SupportStructureGene,
    descriptor: &GraphDescriptor,
    settings: &Settings,
) -> Cost {
    let cost: f32 = gene
        .nodes
        .iter()
        .flat_map(|(_, n)| {
            if let SupportNode::Contact(c) = n {
                Some(c)
            } else {
                None
            }
        })
        .flat_map(|x| {
            x.leans_on.iter().map(|y| {
                cost_of_single_cone(
                    descriptor.details[y].position,
                    x.position,
                    x.radius,
                    settings,
                )
            })
        })
        .sum();
    Cost::new(cost)
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
                .support_structure_refinement_settings
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
                    // approximated surface o the cone plus the sphere area
                    len * radius * 2.0 * std::f32::consts::PI
                        + node_descriptor.radius.powi(2) * 4. * std::f32::consts::PI
                })
                .sum();
            len
        })
        .sum();
    Cost::new(
        surface
            * settings
                .support_structure_refinement_settings
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
    let s = &settings.support_structure_refinement_settings;
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

fn compliance_value(s: &SupportStructureRefinementSettings, stiffness: &Stiffness) -> f32 {
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
    graph: &Graph,
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
    s: &SupportStructureRefinementSettings,
    graph: &mut Graph,
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

fn evaluate_stiffness_cost(
    surface: &SurfaceGraph,
    descriptor: &GraphDescriptor,
    settings: &Settings,
    floating_regions: &[FloatingRegion],
) -> Cost {
    let s = &settings.support_structure_refinement_settings;
    let mut floating_regions_collector = FloatingRegionsCollector::new(floating_regions, surface);
    let mut cost = 0.;
    let mut graph = Graph::new();
    for node in &descriptor.nodes {
        let node_descriptor = &descriptor.details[node];
        let node_position = node_descriptor.position;
        let node_radius = node_descriptor.radius;
        cost += floating_regions_collector.dump_costs(s, node_position.z, &mut graph, surface);
        let supporters: Vec<_> = descriptor.edges[node]
            .iter()
            .filter(|x| graph.nodes.contains_key(*x))
            .collect();
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
    cost += floating_regions_collector.dump_costs(s, f32::MAX, &mut graph, surface);
    Cost::new(cost)
}

fn evaluate_floating_regions_stiffness_cost(
    s: &SupportStructureRefinementSettings,
    mut graph: Graph,
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

    // todo: hard codded value
    graph.add_node(id, center, &neighbors, false, 3.0);

    let stiffness = evaluate_single_node_stiffness(s, &mut graph, id);
    let compliance = compliance_value(s, &stiffness);

    // todo: hard codded value
    (compliance - r.compliance_threshold).max(0.) * 80.
}

fn evaluate_collision_cost(
    descriptor: &GraphDescriptor,
    volume: &Volume,
    settings: &Settings,
) -> Cost {
    let mut cost = 0.;
    let s = &settings.support_structure_refinement_settings;
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
        s: &SupportStructureRefinementSettings,
        until_height: f32,
        graph: &Graph,
        surface: &SurfaceGraph,
    ) -> f32 {
        let mut c = 0.;
        while let Some(e) = self.elements.front() && e.0 < until_height {
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
    gene: &SupportStructureGene,
    surface: &SurfaceGraph,
    volume: &Volume,
    settings: &Settings,
    floating_regions: &[FloatingRegion],
) -> Cost {
    let descriptor = genome_to_graph_descriptor(gene);
    let steepness_cost = evaluate_steepness_cost(&descriptor, settings);
    let length_cost = evaluate_length_cost(&descriptor, settings);
    let stiffness_cost =
        evaluate_stiffness_cost(surface, &descriptor, settings, floating_regions);
    let collision_cost = evaluate_collision_cost(&descriptor, volume, settings);

    steepness_cost + length_cost + stiffness_cost + collision_cost
}
