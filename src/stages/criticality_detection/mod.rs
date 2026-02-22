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
        for (i,t) in graph.iter_triangles().enumerate() {
            let v = t.normal();
            let angle = Point::angle_between(&downward, &t.normal());
            let angle_deg = angle.to_degrees();
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
