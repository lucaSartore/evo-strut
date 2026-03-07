mod models;
mod stages;
mod evolution;
use anyhow::Result;

use crate::{
    models::Settings,
    stages::{
        OrientationBasedCriticalityDetector, OrientationBasedCriticalityEvaluator, Pipeline, PipelineBehaviour, StartedState, contact_point_optimization::SimpleContactPointOptimizer, criticality_grouping::DistanceBasedCriticalityGrouper
    },
};

fn main() -> Result<()> {
    let settings = Settings::default();
    type Behaviour = PipelineBehaviour<
        OrientationBasedCriticalityDetector,
        OrientationBasedCriticalityEvaluator,
        DistanceBasedCriticalityGrouper,
        SimpleContactPointOptimizer
    >;
    Pipeline::<StartedState, Behaviour>::run(settings)?;
    Ok(())
}
