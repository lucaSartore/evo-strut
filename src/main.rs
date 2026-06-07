mod evolution;
mod models;
mod stages;
mod support;

use env_logger::Builder;
use log::{error, LevelFilter};

use crate::{
    models::Settings,
    stages::{
        contact_point_optimization::SimpleContactPointOptimizer,
        floating_region_detection::AreaBasedFloatingRegionDetector,
        contact_points_grouping::SimpleContactPointsGrouper,
        criticality_detection::PropagationBasedCriticalityDetector,
        criticality_grouping::DistanceBasedCriticalityGrouper,
        support_structure_optimization::SimpleSupportStructureOptimizer,
        support_structure_refinement::SimpleSupportStructureRefiner, Pipeline, PipelineBehaviour,
        StartedState,
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
