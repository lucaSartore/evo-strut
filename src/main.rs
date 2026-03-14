mod models;
mod stages;
mod evolution;
use anyhow::Result;
use env_logger::Builder;
use log::{LevelFilter, error};
use rand::distr::uniform::Error;
use rerun::TextLogLevel;

use crate::{
    models::Settings,
    stages::{
        OrientationBasedCriticalityDetector, OrientationBasedCriticalityEvaluator, Pipeline, PipelineBehaviour, StartedState, contact_point_optimization::SimpleContactPointOptimizer, criticality_grouping::DistanceBasedCriticalityGrouper
    },
};

fn main() {

    Builder::new()
        .filter_level(LevelFilter::Error)
        .filter_module("evo_strut", LevelFilter::Debug)
        .init();

    let settings = Settings::default();
    type Behaviour = PipelineBehaviour<
        OrientationBasedCriticalityDetector,
        OrientationBasedCriticalityEvaluator,
        DistanceBasedCriticalityGrouper,
        SimpleContactPointOptimizer
    >;
    let value = Pipeline::<StartedState, Behaviour>::run(settings);

    if let Err(e) = value {
        error!("{e:?}");
    }
}
