use crate::models::{CriticalitySettings, SurfaceGraph};

/// trait that return a "criticality score" of a specific mesh with some specific supports
pub trait CriticalityEvaluator {
    fn evaluate_criticality(
        graph: &SurfaceGraph,
        settings: &CriticalitySettings,
        non_critical_nodes: Vec<usize>
    ) -> f32;
}
