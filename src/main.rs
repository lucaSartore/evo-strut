mod models;
mod stages;
mod evolution;
mod support;
use std::marker::PhantomData;

use env_logger::Builder;
use log::{LevelFilter, error};

use crate::{
    evolution::TournamentBasedCrossoverSelection, models::Settings, stages::{
        OrientationBasedCriticalityDetector, Pipeline, PipelineBehaviour, StartedState, contact_point_optimization::SimpleContactPointOptimizer, criticality_detection::PropagationBasedCriticalityDetector, criticality_grouping::DistanceBasedCriticalityGrouper, support_structure_optimization::{SimpleSupportStructureOptimizer, SupportStructureOptimizer}
    }
};


fn main() {

    Builder::new()
        .filter_level(LevelFilter::Error)
        .filter_module("evo_strut", LevelFilter::Info)
        .init();

    let settings = Settings::default();
    type Behaviour = PipelineBehaviour<
        PropagationBasedCriticalityDetector,
        DistanceBasedCriticalityGrouper,
        SimpleContactPointOptimizer,
        SimpleSupportStructureOptimizer
    >;
    let value = Pipeline::<StartedState, Behaviour>::run(settings);

    if let Err(e) = value {
        error!("{e:?}");
    }
}
