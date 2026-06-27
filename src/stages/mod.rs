use anyhow::Result;
use serde::Serialize;
use std::{
    fs::{self, File},
    io::BufWriter,
    marker::PhantomData,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

pub mod contact_point_optimization;
pub mod contact_points_grouping;
pub mod criticality_detection;
pub mod criticality_grouping;
pub mod exporting;
pub mod floating_region_detection;
pub mod loading;
pub mod support_structure_optimization;
pub mod support_structure_refinement;
pub mod visualization;

pub use criticality_detection::{CriticalityDetectionStage, CriticalityDetector};
use hashbrown::HashSet;
use log::info;

use crate::{
    models::{FaceId, MeshVector, Settings, SurfaceGraph},
    stages::{
        contact_point_optimization::{
            ContactPointOptimizationStage, ContactPointOptimizer, ContactPointsGene,
        },
        contact_points_grouping::{
            ContactPointGroupingGene, ContactPointsGrouper, ContactPointsGroupingStage,
        },
        criticality_grouping::{CriticalityGrouper, CriticalityGroupingStage},
        exporting::ExportingStage,
        floating_region_detection::{
            FloatingRegion, FloatingRegionDetectionStage, FloatingRegionDetector,
        },
        loading::LoadingStage,
        support_structure_optimization::{
            SupportStructureOptimizationStage, SupportStructureOptimizer,
        },
        support_structure_refinement::{
            SupportStructureGene, SupportStructureRefinementStage, SupportStructureRefiner,
        },
    },
};
use visualization::{VisualizationStage, Visualizer};

#[derive(Serialize)]
struct OptimizationArtifact<'a, T>
where
    T: Serialize,
{
    element: &'a T,
    cost_log: &'a crate::evolution::CostLog,
}

pub(crate) fn save_optimization_artifact<T>(
    settings: &Settings,
    file_name: impl AsRef<Path>,
    element: &T,
    cost_log: &crate::evolution::CostLog,
) -> Result<()>
where
    T: Serialize,
{
    let Some(dir) = &settings.io_settings.optimization_logs_dir_path else {
        return Ok(());
    };

    fs::create_dir_all(dir)?;
    let path = Path::new(dir).join(file_name);
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    let artifact = OptimizationArtifact { element, cost_log };
    serde_json::to_writer_pretty(writer, &artifact)?;
    Ok(())
}

pub trait PipelineBehaviourTrait {
    type TCriticalityDetection: CriticalityDetector;
    type TFloatingRegionDetection: FloatingRegionDetector;
    type TCriticalityGrouping: CriticalityGrouper;
    type TContactPointOptimizer: ContactPointOptimizer;
    type TContactPointGrouper: ContactPointsGrouper;
    type TSupportStructureOptimizer: SupportStructureOptimizer;
    type TSupportStructureRefiner: SupportStructureRefiner;
}

pub struct PipelineBehaviour<
    TD: CriticalityDetector,
    TFD: FloatingRegionDetector,
    TG: CriticalityGrouper,
    TCPO: ContactPointOptimizer,
    TCPG: ContactPointsGrouper,
    TSSO: SupportStructureOptimizer,
    TSSR: SupportStructureRefiner,
> {
    _t: PhantomData<(TD, TFD, TG, TCPO, TCPG, TSSO, TSSR)>,
}

impl<
        TCriticalityDetection: CriticalityDetector,
        TFloatingRegionDetection: FloatingRegionDetector,
        TCriticalityGrouping: CriticalityGrouper,
        TContactPointOptimizer: ContactPointOptimizer,
        TContactPointGrouper: ContactPointsGrouper,
        TSupportStructureOptimizer: SupportStructureOptimizer,
        TSupportSTructureRefiner: SupportStructureRefiner,
    > PipelineBehaviourTrait
    for PipelineBehaviour<
        TCriticalityDetection,
        TFloatingRegionDetection,
        TCriticalityGrouping,
        TContactPointOptimizer,
        TContactPointGrouper,
        TSupportStructureOptimizer,
        TSupportSTructureRefiner,
    >
{
    type TCriticalityDetection = TCriticalityDetection;
    type TFloatingRegionDetection = TFloatingRegionDetection;
    type TCriticalityGrouping = TCriticalityGrouping;
    type TContactPointOptimizer = TContactPointOptimizer;
    type TContactPointGrouper = TContactPointGrouper;
    type TSupportStructureOptimizer = TSupportStructureOptimizer;
    type TSupportStructureRefiner = TSupportSTructureRefiner;
}

pub trait PipelineState {}

/// start: we only know the path
pub struct StartedState {
    pub settings: Settings,
}
impl PipelineState for StartedState {}

/// we have successfully loaded the mesh
pub struct LoadedState {
    pub settings: Settings,
    pub graph: SurfaceGraph,
}
impl PipelineState for LoadedState {}

/// we have successfully detected all the nodes that are considered critical
pub struct CriticalityDetectedState {
    pub settings: Settings,
    pub graph: SurfaceGraph,
    pub critical: Vec<FaceId>,
}
impl PipelineState for CriticalityDetectedState {}

/// we have successfully detected all the nodes that are considered critical
pub struct FloatingRegionsDetectedStage {
    pub settings: Settings,
    pub graph: SurfaceGraph,
    pub critical: Vec<FaceId>,
    pub floating_regions: Vec<FloatingRegion>,
}
impl PipelineState for FloatingRegionsDetectedStage {}

/// we have grouped the criticality into areas
pub struct CriticalityGroupedState {
    pub settings: Settings,
    pub graph: SurfaceGraph,
    pub critical: MeshVector<FaceId, bool>,
    pub grouped_areas: Vec<Vec<FaceId>>,
    pub grouped_areas_hashes: Vec<HashSet<FaceId>>,
    pub floating_regions: Vec<FloatingRegion>,
}
impl PipelineState for CriticalityGroupedState {}

/// we have decided how contact points are grouped
pub struct ContactPointsDecidedState {
    pub settings: Settings,
    pub graph: SurfaceGraph,
    pub critical: MeshVector<FaceId, bool>,
    pub connection_points: ContactPointsGene,
    pub floating_regions: Vec<FloatingRegion>,
}
impl PipelineState for ContactPointsDecidedState {}

pub struct ContactPointsGroupedState {
    pub settings: Settings,
    pub graph: SurfaceGraph,
    pub connection_points: ContactPointsGene,
    pub grouper: ContactPointGroupingGene,
    pub floating_regions: Vec<FloatingRegion>,
}
impl PipelineState for ContactPointsGroupedState {}

pub struct SupportStructureOptimizedState {
    pub settings: Settings,
    pub graph: SurfaceGraph,
    pub connection_points: ContactPointsGene,
    pub support_structures: Vec<SupportStructureGene>,
}
impl PipelineState for SupportStructureOptimizedState {}

pub struct SupportStructureRefinedState {
    pub settings: Settings,
    pub graph: SurfaceGraph,
    pub connection_points: ContactPointsGene,
    pub support_structures: Vec<SupportStructureGene>,
}
impl PipelineState for SupportStructureRefinedState {}

pub struct MeshExportedState {}
impl PipelineState for MeshExportedState {}

pub struct Pipeline<TS, TB>
where
    TS: PipelineState,
    TB: PipelineBehaviourTrait,
{
    _b: PhantomData<TB>,
    state: TS,
}

impl<TS, TB> Pipeline<TS, TB>
where
    TB: PipelineBehaviourTrait,
    TS: PipelineState,
{
    pub fn from_state(state: TS) -> Pipeline<TS, TB> {
        Self {
            _b: PhantomData::default(),
            state,
        }
    }
}

macro_rules! timed {
    ($name:literal, $exp:expr) => {{
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let to_return = $exp;
        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let duration = end - start;
        info!("stage {} took {} [s]", $name, duration.as_secs());
        to_return
    }};
}
impl<TB> Pipeline<StartedState, TB>
where
    TB: PipelineBehaviourTrait,
{
    pub fn new(settings: Settings) -> Pipeline<StartedState, TB> {
        Self {
            _b: Default::default(),
            state: StartedState { settings },
        }
    }
    pub fn run(settings: Settings) -> Result<()> {
        let p = Self::new(settings);
        let p = timed!("loading", LoadingStage::<TB>::execute(p))?;
        let p = match p {
            // no pre-loaded mesh, we need to repeat every step
            loading::LoadingStageOutput::MeshLoaded(p) => {
                timed!("visualizing", VisualizationStage::visualize(&p))?;
                let p = timed!(
                    "criticality_detection",
                    CriticalityDetectionStage::<TB>::execute(p)
                );
                timed!("visualizing", VisualizationStage::visualize(&p))?;
                let p = timed!(
                    "floating_region_detection",
                    FloatingRegionDetectionStage::<TB>::execute(p)
                );
                timed!("visualizing", VisualizationStage::visualize(&p))?;
                // return Ok(());
                let p = timed!(
                    "criticality_grouping",
                    CriticalityGroupingStage::<TB>::execute(p)
                );
                timed!("visualizing", VisualizationStage::visualize(&p))?;
                let p = timed!(
                    "contact_points_optimization",
                    ContactPointOptimizationStage::<TB>::execute(p)
                )?;
                timed!("visualizing", VisualizationStage::visualize(&p))?;
                let p = timed!(
                    "contact_points_grouping",
                    ContactPointsGroupingStage::<TB>::execute(p)
                )?;
                let p = timed!(
                    "support_structure_optimization",
                    SupportStructureOptimizationStage::<TB>::execute(p)
                )?;
                timed!(
                    "support_structure_refinement",
                    SupportStructureRefinementStage::<TB>::execute(p)
                )?
            }
            // pre-loaded struct, we can skip to exporting
            loading::LoadingStageOutput::StructureLoaded(p) => p,
        };

        let _ = timed!("exporting", ExportingStage::execute(p))?;
        Ok(())
    }
}
