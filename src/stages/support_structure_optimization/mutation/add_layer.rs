use itertools::Itertools;
use rerun::external::arrow::datatypes::ArrowNativeType;

use crate::{models::Point, stages::support_structure_optimization::models::{LayerNode, SupportGroup, SupportLayer}, support::convex_hull::ConvexHull};

use super::*;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut CompressedSupportGene) {
    let rand = &mutator.rand;
    let group = gene.rand_group_mut(rand);
    let max_height = group.max_height();
    let layer_height = rand.next_f32(0., max_height);

    add_layer(group, layer_height, mutator);
}

pub fn add_layer(group: &mut SupportGroup, layer_height: f32, mutator: &SupportStructureMutator) {
    let s = &mutator.settings.support_structure_optimization_settings;
    let rand = &mutator.rand;


    let points = group.points_to_support_above(layer_height);

    let random_point = || *rand.choose(&points).expect("the points to support can't be empty");

    let hull = ConvexHull::new(points.clone());

    let num_points = (
        s.point_in_layer_density * hull.area() +
        s.point_in_layer_perimeter_density * hull.perimeter()
    ).as_usize().max(s.min_points_in_layer);
    
    let mut center = Point::mean(&points);
    center.z = layer_height;

    let layer = SupportLayer {
        center,
        nodes: (0..num_points).map(|_| {
            let base_node = random_point();
            let direction_one = (random_point() - random_point()).as_versor();
            let direction_two = (base_node - random_point()).as_versor();
            let mut direction_three = Point::random(Point::ZERO, 1., rand);
            direction_three.z = 0.;
            direction_three = direction_three.as_versor();
            let new_node = base_node + (direction_one + direction_two + direction_three).to_scaled(s.layer_node_creation_update_step);
            let mut versor = new_node - center;
            versor.z = 0.;
            versor
        })
        .unique()
        .map(|x| LayerNode::new_random(x, rand))
        .collect()
    };

    group.layers.push(layer);
}
