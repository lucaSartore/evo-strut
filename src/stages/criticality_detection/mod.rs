use crate::models::{Settings, SurfaceGraph};



/// trait that given a particular mesh detect which polygons are "critical"
pub trait CriticalityDetector {
    fn detect_criticality(graph: SurfaceGraph, settings: Settings) -> Vec<usize>;
}

pub struct OrientationBasedCriticality{}

impl CriticalityDetector for OrientationBasedCriticality {
    fn detect_criticality(graph: SurfaceGraph, settings: Settings) -> Vec<usize> {
        let to_return = vec![];
        for node in graph.nodes.iter() {
        }

        to_return
    }
}
