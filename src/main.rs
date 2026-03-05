mod models;
mod stages;
use anyhow::Result;

use crate::{
    models::Settings,
    stages::{
        OrientationBasedCriticalityDetector, OrientationBasedCriticalityEvaluator, Pipeline,
        PipelineBehaviour, StartedState, criticality_grouping::DistanceBasedCriticalityGrouper,
    },
};

fn main() -> Result<()> {
    let settings = Settings::default();
    type Behaviour = PipelineBehaviour<
        OrientationBasedCriticalityDetector,
        OrientationBasedCriticalityEvaluator,
        DistanceBasedCriticalityGrouper
    >;
    Pipeline::<StartedState, Behaviour>::run(settings)?;
    Ok(())
}
