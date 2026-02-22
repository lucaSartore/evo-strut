use rerun::demo_util::grid;

use crate::models::{Point, Settings, SurfaceGraph};



/// trait that given a particular mesh detect which polygons are "critical"
pub trait CriticalityDetector {
    fn detect_criticality(graph: &SurfaceGraph, settings: &Settings) -> Vec<usize>;
}

pub struct OrientationBasedCriticality{}

impl CriticalityDetector for OrientationBasedCriticality {
    fn detect_criticality(graph: &SurfaceGraph, settings: &Settings) -> Vec<usize> {
        let mut to_return = vec![];
        let downward = Point{x:0., y:0., z:-1.};
        'triangles_loop:
        for (i,t) in graph.iter_triangles().enumerate() {
            
            // filter out the triangles that are touching the ground (and therefore
            // are already supported)
            for v in t.vertexes() {
                if v.z <= settings.criticality_settings.max_detachment_from_z_plane {
                    continue 'triangles_loop
                }
            }

            // if a triangle has no neighbor that is lower than him, than it is also 
            // a critical
            let mut has_lower_neighbor = false;
            for adj_id in graph.nodes[i].adjacent.iter() {
                let adj = graph.get_triangle(*adj_id);
                let distance = t.center() - adj.center();
                if distance.z > 0. {
                    has_lower_neighbor = true;
                }
            }

            // calculating the angle of the surface w.r.t. the vector facing downward
            let angle = Point::angle_between(&downward, &t.normal());
            let angle_deg = angle.to_degrees();

            // condition based on the fact that the current point has
            // no neighbor that is lower than self
            if !has_lower_neighbor && angle_deg <= 90. {
                to_return.push(i);
                continue 'triangles_loop
            }
            
            // condition based purely on the angle of the surface
            // if zero everything is supported, if set to 90 nothing is supported
            let threshold = settings.criticality_settings.support_overhanging_angle;
            // if 90 everything is supported, if zero nothing is supported
            let inverted_threshold = 90. - threshold;
            if angle_deg < inverted_threshold {
                to_return.push(i);
            }
        }
        to_return
    }
}
