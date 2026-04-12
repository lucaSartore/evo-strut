use hashbrown::HashMap;
use nalgebra::{ArrayStorage, Matrix6, distance};
use rerun::external::arrow::ipc::convert::try_schema_from_ipc_buffer;
use smallvec::{SmallVec, smallvec};
use crate::models::MaterialStiffnessSettings;

use crate::stages::support_structure_optimization::evaluation::stiffness::calculate_stiffness;
use crate::{evolution::Cost, models::{Point, PointId}, stages::support_structure_optimization::{ContactNode, SupportNode, SupportNodeId, evaluation::{graph::Graph, stiffness::{ Stiffness, stiffness_parallel, stiffness_series}}}};

use super::*;


pub struct GraphDescriptor {
    /// nodes with relative position ordered by height
    pub nodes: Vec<SupportNodeId>,
    pub positions: HashMap<SupportNodeId, Point>,
    pub edges: HashMap<SupportNodeId, SmallVec<[SupportNodeId;4]>>
}

fn genome_to_graph_descriptor(gene: &SupportStructureGene) -> GraphDescriptor {
    let mut nodes = vec![];
    let mut positions = HashMap::new();
    let mut edges = HashMap::new();
    for (_,g) in gene.nodes.iter() {
        let (node_id, position, adj) = match g {
            SupportNode::Contact(_) => continue,
            SupportNode::Base(n) => (n.id, n.last_position, &smallvec![]),
            SupportNode::Middle(n) => (n.id, n.last_position, &n.leans_on)
        };
        nodes.push(node_id);
        positions.insert(node_id, position);
        for n in adj {
            edges.entry(*n)
                .or_insert(smallvec![])
                .push(node_id);
        }
        edges.entry(node_id)
            .or_insert(smallvec![])
            .append(&mut adj.clone());
    }

    nodes.sort_by_key(|x| Cost::new(positions[x].z));

    GraphDescriptor { nodes, positions, edges }
}


fn cost_of_single_cone(base: Point, circle_center: Point, circle_radius: f32, settings: &Settings) -> f32 {
    let s = &settings.support_structure_optimization_settings;
    if base.z >= circle_center.z {
        return s.cost_of_un_feasible_cone;
    }
    let vec = circle_center - base;
    let height = vec.z.abs();

    let base_offset = (vec.x.powi(2) + vec.y.powi(2)).sqrt();
    
    //approximated area of the cone 
    let side = ((base_offset + circle_radius).powi(2) + height.powi(2)).sqrt();
    let area = side * circle_radius * std::f32::consts::PI;

    let steepness = f32::atan(height / (circle_radius + base_offset));
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

fn evaluate_cones_cost(gene: &SupportStructureGene, descriptor: &GraphDescriptor, settings: &Settings) -> Cost {
    let cost: f32 = gene
        .nodes
        .iter()
        .flat_map(|(_,n)| {
            if let SupportNode::Contact(c) = n {
                Some(c)
            } else {
                None
            }
        })
        .flat_map(|x| {
            x.leans_on
                .iter()
                .map(|y| {
                    cost_of_single_cone(
                        descriptor.positions[y],
                        x.position,
                        x.radius,
                        settings
                    )
                })
        }).sum();
    Cost::new(cost)
}

fn evaluate_steepness_cost(descriptor: &GraphDescriptor, settings: &Settings) -> Cost {
    let threshold = 90. - settings.criticality_settings.support_overhanging_angle;
    let distance_times_angle: f32 = descriptor
        .edges
        .iter()
        .map(|(node, neighbours)| {
            let node_position = descriptor.positions[node];
            let len: f32 = neighbours
                .iter()
                .filter(|neighbour| **neighbour < *node)
                .map(|neighbour| {
                    let neighbour_position = descriptor.positions[neighbour];
                    let angle = Point::horizon_angle(node_position, neighbour_position).to_degrees().abs();
                    let distance = (node_position - neighbour_position).abs();
                    let angle_diff = angle - settings.criticality_settings.support_overhanging_angle;
                    if angle_diff < threshold {
                        return (threshold - angle_diff) * distance;
                    }
                    0.0
                }).sum();
            len
        }).sum();
    Cost::new(distance_times_angle * settings.support_structure_optimization_settings.cost_for_support_too_steep)
}

fn evaluate_length_cost(descriptor: &GraphDescriptor, settings: &Settings) -> Cost {
    let distance: f32 = descriptor
        .edges
        .iter()
        .map(|(node, neighbours)| {
            let node_position = descriptor.positions[node];
            let len: f32 = neighbours
                .iter()
                .filter(|neighbour| **neighbour < *node)
                .map(|neighbour| {
                    let neighbour_position = descriptor.positions[neighbour];
                    (node_position - neighbour_position).abs()
                }).sum();
            len
        }).sum();
    Cost::new(distance * settings.support_structure_optimization_settings.cost_for_unit_of_length)
}

fn evaluate_single_support_stiffness(base_stiffness: &Stiffness, from: Point, to: Point, settings: &Settings) -> f32 {
    let s = &settings.support_structure_optimization_settings;
    let to_integrate = Point::interpolate(from, to, s.stiffness_cost_integration_size);
    let vector = to - from;
    let mut cost = 0.;
    for i in 1..to_integrate.len() {
        let scalar = i as f32 / (to_integrate.len() - 1) as f32;
        let new_to = from + vector.to_scaled(scalar);
        let stiffness = stiffness_series(base_stiffness, from, new_to, &s.material_stiffness_settings);
        let sxx = stiffness.0[(0,0)];
        let syy = stiffness.0[(1,1)];
        cost += s.non_stiffness_cost * sxx + s.non_stiffness_cost * syy
    }
    cost
}

fn evaluate_stiffness<'b, 'a: 'b>(point: SupportNodeId, graph: &Graph, cache: &'a mut HashMap<SupportNodeId, Stiffness>, settings: &MaterialStiffnessSettings) {
    if cache.contains_key(&point) {
        return
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
        stiffness_to_combine.push(
            stiffness_series(
                &weighted_stiffness,
                graph.nodes[&support].position,
                node_position,
                settings
            )
        );
    }
    let final_stiffness = stiffness_parallel(&stiffness_to_combine);

    cache.insert(point, final_stiffness);
}

fn evaluate_stiffness_cost(gene: &SupportStructureGene, descriptor: &GraphDescriptor, settings: &Settings) -> Cost {
    let s = &settings.support_structure_optimization_settings;
    let mut cost = 0.;
    let mut graph = Graph::new();
    for node in &descriptor.nodes {
        let node_position = descriptor.positions[node];
        let supporters: Vec<_> = descriptor
            .edges[node]
            .iter()
            .filter(|x| graph.nodes.contains_key(*x))
            .collect();
        for supporter in supporters {
            let supporter_position = descriptor.positions[supporter];
            let visited_nodes = graph.mark_distances(*supporter);
            let mut cache = HashMap::default();
            evaluate_stiffness(*supporter, &graph, &mut cache, &s.material_stiffness_settings);
            let base_stiffness = &cache[supporter];
            cost += evaluate_single_support_stiffness(base_stiffness, supporter_position, node_position, settings);
            graph.reset_some_nodes(&visited_nodes);
        }
        graph.add_node(
            *node,
            descriptor.positions[node],
            &descriptor.edges[node],
            gene.is_supported(*node)
        );
    }
    Cost::new(cost)
}

pub fn evaluate_cost(gene: &SupportStructureGene, settings: &Settings) -> Cost {
    let descriptor = genome_to_graph_descriptor(gene);
    let mut cost = Cost::ZERO;
    cost = cost + evaluate_cones_cost(gene, &descriptor, settings);
    cost = cost + evaluate_steepness_cost(&descriptor, settings);
    cost = cost + evaluate_length_cost(&descriptor, settings);
    cost = cost + evaluate_stiffness_cost(gene, &descriptor, settings);
    cost
}
