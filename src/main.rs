mod models;
mod stages;
use anyhow::Result;

use crate::{
    models::Settings,
    stages::{
        OrientationBasedCriticalityDetector, OrientationBasedCriticalityEvaluator, Pipeline,
        PipelineBehaviour, StartedState,
    },
};

fn main() -> Result<()> {
    let settings = Settings::default();
    type Behaviour = PipelineBehaviour<
        OrientationBasedCriticalityDetector,
        OrientationBasedCriticalityEvaluator,
    >;
    Pipeline::<StartedState, Behaviour>::run(settings)?;
    Ok(())
}
