use crate::models::{CriticalitySettings, SurfaceGraph};
use std::marker::PhantomData;

pub  struct CriticalityEvaluationStage<T>
where
    T: CriticalityEvaluator
{
    _d: PhantomData<T>
}

pub struct Criticality {
    /// the score associated with the criticality
    score: f32,
    /// the area that generated the critical aspect
    area: f32,
    /// does the current area have some connected meshes below
    /// (that can support it) or is it printed on nothing?
    supported: bool
}

/// trait that return a "criticality score" of a specific mesh with some specific supports
pub trait CriticalityEvaluator {
    fn evaluate_criticality(
        graph: &SurfaceGraph,
        settings: &CriticalitySettings,
        non_critical_nodes: Vec<usize>
    ) -> Criticality;
}

pub struct OrientationBasedCriticalityEvaluator {}

impl CriticalityEvaluator for OrientationBasedCriticalityEvaluator {
    fn evaluate_criticality(
        graph: &SurfaceGraph,
        settings: &CriticalitySettings,
        non_critical_nodes: Vec<usize>
    ) -> Criticality {
        todo!()
    }
}
