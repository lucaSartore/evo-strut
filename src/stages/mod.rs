use std::{marker::PhantomData, process::exit};
use anyhow::Result;

pub mod loading;
pub mod visualization;
pub mod criticality_evaluation;
pub mod criticality_detection;

pub use criticality_detection::{CriticalityDetector, CriticalityDetectionStage, OrientationBasedCriticalityDetector};
pub use criticality_evaluation::{CriticalityEvaluator, CriticalityEvaluationStage, OrientationBasedCriticalityEvaluator};
use stl_io::IndexedMesh;

use crate::{models::{Settings, SurfaceGraph}, stages::{loading::LoadingStage}};
use visualization::{VisualizationStage, Visualizer};

pub trait PipelineBehaviourTrait {
    type TCriticalityDetection: CriticalityDetector;
    type TCriticalityEvaluation: CriticalityEvaluator;
}

pub struct PipelineBehaviour<
    TD: CriticalityDetector,
    TE: CriticalityEvaluator
> {
    _t: PhantomData<(
        TD,
        TE
    )>
}

impl<
    TCriticalityDetection: CriticalityDetector,
    TCriticalityEvaluation: CriticalityEvaluator
> PipelineBehaviourTrait for PipelineBehaviour<
    TCriticalityDetection,
    TCriticalityEvaluation
> {
    type TCriticalityDetection = TCriticalityDetection;
    type TCriticalityEvaluation = TCriticalityEvaluation;
}

pub trait PipelineState {}

/// start: we only know the path 
pub struct StartedState {
    pub settings: Settings
}
impl PipelineState for StartedState { }

/// we have successfully loaded the mesh
pub struct LoadedState {
    pub settings: Settings,
    pub graph: SurfaceGraph
}
impl PipelineState for LoadedState { }

/// we have successfully detected all the nodes that are considered critical
pub struct CriticalityDetectedState {
    pub settings: Settings,
    pub graph: SurfaceGraph,
    pub critical: Vec<usize>
}
impl  PipelineState for CriticalityDetectedState { }

pub struct Pipeline<TS, TB> 
where 
    TS: PipelineState,
    TB: PipelineBehaviourTrait
{
    _b: PhantomData<TB>,
    state: TS
}

impl<TS, TB> Pipeline<TS, TB>
where 
    TB: PipelineBehaviourTrait,
    TS: PipelineState,
{
    pub fn from_state(state: TS) -> Pipeline<TS,TB> {
        Self {
            _b: PhantomData::default(),
            state
        }
    }

}
impl<TB> Pipeline<StartedState, TB>
where 
    TB: PipelineBehaviourTrait,
{
    pub fn new(settings: Settings) -> Pipeline<StartedState,TB> {
        Self {
            _b: PhantomData::default(),
            state: StartedState { settings }
        }
    }
    pub fn run(settings: Settings) -> Result<()> {
        let p = Self::new(settings);
        let p = LoadingStage::<TB>::execute(p)?;
        VisualizationStage::visualize(&p)?;
        let p = CriticalityDetectionStage::<TB>::execute(p);
        VisualizationStage::visualize(&p)?;
        return Ok(())
    }
}
