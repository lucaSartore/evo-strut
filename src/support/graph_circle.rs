use std::collections::VecDeque;

use baby_shark::data_structures::st_tree::NodeIndex;
use hashbrown::HashSet;

use crate::models::{FaceId, SurfaceGraph};




/// find all the nodes that are closer to the center of a certain radius
/// effectively creating a circle around the center node.
pub fn find_circle(graph: &SurfaceGraph, center: FaceId, radius: f32) -> HashSet<FaceId> {
    let mut set = HashSet::default();
    let mut to_visit = Vec::default();

    set.insert(center);
    to_visit.push(center);
    let circle_center = graph.get_triangle(center).center();

    while let Some(node_id) = to_visit.pop() {
        for adj in graph.iter_adjacent(node_id) {
            // skip nodes already processed
            if set.contains(&adj.index) {
                continue;
            }
            // skip nodes that are too far
            let distance = (adj.center() - circle_center).abs();
            if distance > radius {
                continue
            }
            // process the rest
            set.insert(adj.index);
            to_visit.push(adj.index);
        }
    }
    set
}
