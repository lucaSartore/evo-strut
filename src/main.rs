mod models;
mod stages;
mod evolution;
use anyhow::Result;
use env_logger::Builder;
use log::LevelFilter;

use crate::{
    models::Settings,
    stages::{
        OrientationBasedCriticalityDetector, OrientationBasedCriticalityEvaluator, Pipeline, PipelineBehaviour, StartedState, contact_point_optimization::SimpleContactPointOptimizer, criticality_grouping::DistanceBasedCriticalityGrouper
    },
};

fn main() -> Result<()> {

    Builder::new()
        .filter_level(LevelFilter::Info)
        .init();

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
