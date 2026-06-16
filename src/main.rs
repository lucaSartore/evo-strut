mod evolution;
mod models;
mod stages;
mod support;

use env_logger::Builder;
use log::{LevelFilter, error};

use crate::{
    models::Settings,
    stages::{
        Pipeline, PipelineBehaviour, StartedState,
        contact_point_optimization::SimpleContactPointOptimizer,
        contact_points_grouping::SimpleContactPointsGrouper,
        criticality_detection::PropagationBasedCriticalityDetector,
        criticality_grouping::DistanceBasedCriticalityGrouper,
        floating_region_detection::AreaBasedFloatingRegionDetector,
        support_structure_optimization::SimpleSupportStructureOptimizer,
        support_structure_refinement::SimpleSupportStructureRefiner,
    },
};

fn main() {
    Builder::new()
        .filter_level(LevelFilter::Error)
        .filter_module("evo_strut", LevelFilter::Info)
        .init();

    let settings = Settings::default();
    type Behaviour = PipelineBehaviour<
        PropagationBasedCriticalityDetector,
        AreaBasedFloatingRegionDetector,
        DistanceBasedCriticalityGrouper,
        SimpleContactPointOptimizer,
        SimpleContactPointsGrouper,
        SimpleSupportStructureOptimizer,
        SimpleSupportStructureRefiner,
    >;
    let value = Pipeline::<StartedState, Behaviour>::run(settings);

    if let Err(e) = value {
        error!("{e:?}");
    }
}
