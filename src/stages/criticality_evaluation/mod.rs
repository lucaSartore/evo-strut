use crate::models::{CriticalitySettings, SurfaceGraph};

pub trait CriticalityEvaluator {
    fn evaluate_criticality(graph: &SurfaceGraph,
        settings: &CriticalitySettings,
        non_critical_nodes: Vec<usize>
    ) -> f32;
}
