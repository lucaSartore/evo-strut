use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use smallvec::{smallvec, SmallVec};
use std::{marker::PhantomData, path::Iter};

use crate::{
    evolution::Cost,
    models::{FaceId, Point, PointId, Settings, SurfaceGraph, Triangle},
    stages::{
        CriticalityDetectedState, FloatingRegionsDetectedStage, LoadedState, Pipeline, PipelineBehaviourTrait, criticality_detection::propagation::{CostWithArea, KnownCosts}
    },
};

pub struct FloatingRegionDetectionStage<TB>
where
    TB: PipelineBehaviourTrait,
{
    _d: PhantomData<TB>,
}

impl<TB> FloatingRegionDetectionStage<TB>
where
    TB: PipelineBehaviourTrait,
{
    pub fn execute(input: Pipeline<CriticalityDetectedState, TB>) -> Pipeline<FloatingRegionsDetectedStage, TB> {
        let graph = &input.state.graph;
        let settings = &input.state.settings;
        let floating_regions = TB::TFloatingRegionDetection::detect_floating_regions(graph, settings);
        Pipeline::from_state(FloatingRegionsDetectedStage {
            settings: input.state.settings,
            graph: input.state.graph,
            critical: input.state.critical,
            floating_regions
        })
    }
}

pub struct FloatingRegion {
    faces: HashSet<FaceId>,
    stiffness_threshold: f32
}

/// trait that given a particular mesh detect which polygons are "critical"
pub trait FloatingRegionDetector {
    fn detect_floating_regions(graph: &SurfaceGraph, settings: &Settings) -> Vec<FloatingRegion>;
}
fn is_triangle_close_to_the_ground(triangle: &Triangle<'_>, settings: &Settings) -> bool {
    triangle.center().z <= settings.criticality_settings.max_detachment_from_z_plane
}

pub struct AreaBasedFloatingRegionDetector {
}


impl AreaBasedFloatingRegionDetector {
    fn neighbors_same_layer_or_above<'a>(t: Triangle<'a>, graph: &'a SurfaceGraph, settings: &'a Settings) -> impl Iterator<Item = Triangle<'a>> {
        let layer = t.center().layer(settings);
        graph
            .neighbors_ref(t.index)
            .iter()
            .map(|x| graph.get_triangle(*x))
            .filter(move |x| x.center().layer(settings) >= layer)
    }

    fn are_to_stiffness_threshold(area: f32, settings: &Settings) -> f32 {
        // add a new session in settings, called "floating region detection,
        // with a constant that is used to multiply the area to calculate the stiffness threshold
        todo!();
    }
}

impl FloatingRegionDetector for AreaBasedFloatingRegionDetector {
    fn detect_floating_regions(graph: &SurfaceGraph, settings: &Settings) -> Vec<FloatingRegion> {
        // here some examples of the ABI
        for t in graph.iter_triangles(None) {
            for n in Self::neighbors_same_layer_or_above(t, graph, settings) {
            }
        }
        todo!()
    }
}
