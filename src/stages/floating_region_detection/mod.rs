use hashbrown::HashSet;
use std::marker::PhantomData;

use crate::{
    models::{FaceId, Settings, SurfaceGraph, Triangle},
    stages::{
        CriticalityDetectedState, FloatingRegionsDetectedStage, Pipeline, PipelineBehaviourTrait,
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
    pub fn execute(
        input: Pipeline<CriticalityDetectedState, TB>,
    ) -> Pipeline<FloatingRegionsDetectedStage, TB> {
        let graph = &input.state.graph;
        let settings = &input.state.settings;
        let floating_regions =
            TB::TFloatingRegionDetection::detect_floating_regions(graph, settings);
        Pipeline::from_state(FloatingRegionsDetectedStage {
            settings: input.state.settings,
            graph: input.state.graph,
            critical: input.state.critical,
            floating_regions,
        })
    }
}

pub struct FloatingRegion {
    faces: HashSet<FaceId>,
    stiffness_threshold: f32,
}

impl FloatingRegion {
    pub fn faces(&self) -> &HashSet<FaceId> {
        &self.faces
    }

    pub fn stiffness_threshold(&self) -> f32 {
        self.stiffness_threshold
    }
}

/// trait that given a particular mesh detect which polygons are "critical"
pub trait FloatingRegionDetector {
    fn detect_floating_regions(graph: &SurfaceGraph, settings: &Settings) -> Vec<FloatingRegion>;
}

pub struct AreaBasedFloatingRegionDetector {}

impl AreaBasedFloatingRegionDetector {
    fn neighbors_same_layer_or_above<'a>(
        t: Triangle<'a>,
        graph: &'a SurfaceGraph,
        settings: &'a Settings,
    ) -> impl Iterator<Item = Triangle<'a>> {
        let layer = t.center().layer(settings);
        graph
            .neighbors_ref(t.index)
            .iter()
            .map(|x| graph.get_triangle(*x))
            .filter(move |x| x.center().layer(settings) >= layer)
    }

    fn area_to_stiffness_threshold(area: f32, settings: &Settings) -> f32 {
        area * settings
            .floating_region_detection_settings
            .stiffness_threshold_area_multiplier
    }

    fn is_point_close_to_the_ground(t: Triangle<'_>, settings: &Settings) -> bool {
        t.center().z <= settings.criticality_settings.max_detachment_from_z_plane
    }
}

impl FloatingRegionDetector for AreaBasedFloatingRegionDetector {
    fn detect_floating_regions(graph: &SurfaceGraph, settings: &Settings) -> Vec<FloatingRegion> {
        let mut ordered_nodes: Vec<_> = graph.iter_triangles(None).collect();
        ordered_nodes.sort_by(|a, b| a.center().z.total_cmp(&b.center().z));

        let mut visited = HashSet::new();
        let mut floating_regions = Vec::new();

        for source in ordered_nodes {
            if visited.contains(&source.index) {
                continue;
            }

            let source_is_close_to_ground = Self::is_point_close_to_the_ground(source, settings);
            let mut region_faces = HashSet::new();
            let mut region_area = 0.;
            let mut to_visit = vec![source];
            visited.insert(source.index);

            while let Some(current) = to_visit.pop() {
                if !source_is_close_to_ground {
                    region_area += current.area();
                    region_faces.insert(current.index);
                }

                for neighbor in Self::neighbors_same_layer_or_above(current, graph, settings) {
                    if visited.insert(neighbor.index) {
                        to_visit.push(neighbor);
                    }
                }
            }

            if !source_is_close_to_ground {
                floating_regions.push(FloatingRegion {
                    stiffness_threshold: Self::area_to_stiffness_threshold(region_area, settings),
                    faces: region_faces,
                });
            }
        }

        floating_regions
    }
}
