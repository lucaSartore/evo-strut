use std::{marker::PhantomData, process::exit};
use anyhow::Result;

pub mod loading;
pub mod visualization;
pub mod criticality_evaluation;
pub mod criticality_detection;
pub mod criticality_grouping;
pub mod contact_point_optimization;

pub use criticality_detection::{CriticalityDetector, CriticalityDetectionStage, OrientationBasedCriticalityDetector};
pub use criticality_evaluation::{CriticalityEvaluator, CriticalityEvaluationStage, OrientationBasedCriticalityEvaluator};
use rerun::external::image::imageops::FilterType::Triangle;
use stl_io::IndexedMesh;

use crate::{models::{Settings, SurfaceGraph, TriangleId}, stages::{contact_point_optimization::{ContactPointOptimizationStage, ContactPointOptimizer, ContactPointsGene}, criticality_grouping::{CriticalityGrouper, CriticalityGroupingStage}, loading::LoadingStage}};
use visualization::{VisualizationStage, Visualizer};

pub trait PipelineBehaviourTrait {
    type TCriticalityDetection: CriticalityDetector;
    type TCriticalityEvaluation: CriticalityEvaluator;
    type TCriticalityGrouping: CriticalityGrouper;
    type TContactPointOptimizer: ContactPointOptimizer;
}

pub struct PipelineBehaviour<
    TD: CriticalityDetector,
    TE: CriticalityEvaluator,
    TG: CriticalityGrouper,
    TCPO: ContactPointOptimizer
> {
    _t: PhantomData<(
        TD,
        TE,
        TG,
        TCPO
    )>
}

impl<
    TCriticalityDetection: CriticalityDetector,
    TCriticalityEvaluation: CriticalityEvaluator,
    TCriticalityGrouping: CriticalityGrouper,
    TContactPointOptimizer: ContactPointOptimizer
> PipelineBehaviourTrait for PipelineBehaviour<
    TCriticalityDetection,
    TCriticalityEvaluation,
    TCriticalityGrouping,
    TContactPointOptimizer
> {
    type TCriticalityDetection = TCriticalityDetection;
    type TCriticalityEvaluation = TCriticalityEvaluation;
    type TCriticalityGrouping = TCriticalityGrouping;
    type TContactPointOptimizer = TContactPointOptimizer;
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
    pub critical: Vec<TriangleId>
}
impl  PipelineState for CriticalityDetectedState { }

/// we have grouped the criticality into areas
pub struct CriticalityGroupedState {
    pub settings: Settings,
    pub graph: SurfaceGraph,
    pub critical: Vec<Vec<TriangleId>>
}
impl  PipelineState for CriticalityGroupedState { }

/// we have grouped the criticality into areas
pub struct ContactPointsDecidedState {
    pub settings: Settings,
    pub graph: SurfaceGraph,
    pub connection_points: ContactPointsGene
}
impl  PipelineState for ContactPointsDecidedState { }

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
        let p = CriticalityGroupingStage::<TB>::execute(p);
        VisualizationStage::visualize(&p)?;
        let p = ContactPointOptimizationStage::<TB>::execute(p);
        Ok(())
    }
}
