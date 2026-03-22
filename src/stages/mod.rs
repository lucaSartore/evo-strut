use std::marker::PhantomData;
use anyhow::Result;

pub mod loading;
pub mod visualization;
pub mod criticality_detection;
pub mod criticality_grouping;
pub mod contact_point_optimization;

pub use criticality_detection::{CriticalityDetector, CriticalityDetectionStage, OrientationBasedCriticalityDetector};
use hashbrown::HashSet;

use crate::{models::{FaceId, MeshVector, Settings, SurfaceGraph}, stages::{contact_point_optimization::{ContactPointOptimizationStage, ContactPointOptimizer, ContactPointsGene}, criticality_grouping::{CriticalityGrouper, CriticalityGroupingStage}, loading::LoadingStage}};
use visualization::{VisualizationStage, Visualizer};

pub trait PipelineBehaviourTrait {
    type TCriticalityDetection: CriticalityDetector;
    type TCriticalityGrouping: CriticalityGrouper;
    type TContactPointOptimizer: ContactPointOptimizer;
}

pub struct PipelineBehaviour<
    TD: CriticalityDetector,
    TG: CriticalityGrouper,
    TCPO: ContactPointOptimizer
> {
    _t: PhantomData<(
        TD,
        TG,
        TCPO
    )>
}

impl<
    TCriticalityDetection: CriticalityDetector,
    TCriticalityGrouping: CriticalityGrouper,
    TContactPointOptimizer: ContactPointOptimizer
> PipelineBehaviourTrait for PipelineBehaviour<
    TCriticalityDetection,
    TCriticalityGrouping,
    TContactPointOptimizer
> {
    type TCriticalityDetection = TCriticalityDetection;
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
    pub critical: Vec<FaceId>
}
impl  PipelineState for CriticalityDetectedState { }

/// we have grouped the criticality into areas
pub struct CriticalityGroupedState {
    pub settings: Settings,
    pub graph: SurfaceGraph,
    pub critical: MeshVector<FaceId, bool>,
    pub grouped_areas: Vec<Vec<FaceId>>,
    pub grouped_areas_hashes: Vec<HashSet<FaceId>>
}
impl  PipelineState for CriticalityGroupedState { }

/// we have grouped the criticality into areas
pub struct ContactPointsDecidedState {
    pub settings: Settings,
    pub graph: SurfaceGraph,
    pub critical: MeshVector<FaceId, bool>,
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
        let p = ContactPointOptimizationStage::<TB>::execute(p)?;
        VisualizationStage::visualize(&p)?;
        Ok(())
    }
}
