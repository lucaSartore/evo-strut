use itertools::Itertools;
use rerun::external::arrow::datatypes::ArrowNativeType;

use crate::{models::Point, stages::support_structure_optimization::models::{LayerNode, SupportGroup, SupportLayer}, support::convex_hull::ConvexHull};

use super::*;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut SupportGroup) {
    let rand = &mutator.rand;
    let max_height = gene.max_height();
    let layer_height = rand.next_f32(0., max_height);

    add_layer(gene, layer_height, mutator);
}

pub fn add_layer(group: &mut SupportGroup, layer_height: f32, mutator: &SupportStructureMutator) {
    let s = &mutator.settings.support_structure_optimization_settings;
    let rand = &mutator.rand;

    let points = group.points_to_support_above(layer_height);

    let hull = ConvexHull::new(points.clone());

    let num_points = (
        s.point_in_layer_density * hull.area() +
        s.point_in_layer_perimeter_density * hull.perimeter()
    ).as_usize().max(s.min_points_in_layer);
    
    let mut center = Point::mean(&points);
    center.z = layer_height;

    let layer = SupportLayer {
        center,
        nodes: (0..num_points).map(|_| SupportLayer::random_point(layer_height, &points, Some(center), rand, s.layer_node_creation_update_step))
        .unique()
        .map(|x| LayerNode::new_random(x, rand))
        .collect()
    };

    group.layers.push(layer);
}
