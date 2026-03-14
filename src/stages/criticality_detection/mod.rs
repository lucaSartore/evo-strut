use std::marker::PhantomData;

use crate::{models::{Point, Settings, SurfaceGraph, Triangle, TriangleId}, stages::{CriticalityDetectedState, LoadedState, Pipeline, PipelineBehaviourTrait}};


pub struct CriticalityDetectionStage<TB>
where
    TB: PipelineBehaviourTrait,
{
    _d: PhantomData<TB>,
}

impl<TB> CriticalityDetectionStage<TB>
where
    TB: PipelineBehaviourTrait,
{
    pub fn execute(
        input: Pipeline<LoadedState, TB>
    ) -> Pipeline<CriticalityDetectedState, TB> {
        let graph = &input.state.graph;
        let settings = &input.state.settings;
        let critical_nodes = TB::TCriticalityDetection::detect_criticality(graph, settings);
        Pipeline::from_state(CriticalityDetectedState {
            settings: input.state.settings,
            graph: input.state.graph,
            critical: critical_nodes
        })
    }
}

/// trait that given a particular mesh detect which polygons are "critical"
pub trait CriticalityDetector {
    fn detect_criticality(graph: &SurfaceGraph, settings: &Settings) -> Vec<TriangleId>;
}

pub struct OrientationBasedCriticalityDetector {}

fn is_triangle_close_to_the_ground(triangle: &Triangle<'_>, settings: &Settings) -> bool {
    for v in triangle.vertexes() {
        if v.z <= settings.criticality_settings.max_detachment_from_z_plane {
            return true;
        }
    }
    false
}

impl CriticalityDetector for OrientationBasedCriticalityDetector {
    fn detect_criticality(graph: &SurfaceGraph, settings: &Settings) -> Vec<TriangleId> {
        let mut to_return = vec![];
        for t in graph.iter_triangles(None) {
            if is_triangle_close_to_the_ground(&t, settings) {
                continue;
            }

            let i = t.index;
            // if a triangle has no neighbor that is lower than him, than it is also
            // a critical
            let mut has_lower_neighbor = false;
            for adj in graph.iter_adjacent(i) {
                if adj.is_lower_than(&t) {
                    has_lower_neighbor = true;
                    break;
                }
            }

            // calculating the angle of the surface w.r.t. the vector facing downward
            let angle = Point::angle_between(&Point::DOWNWARD, &t.normal());
            let angle_deg = angle.to_degrees();

            // condition based on the fact that the current point has
            // no neighbor that is lower than self
            if !has_lower_neighbor && angle_deg <= 90. {
                to_return.push(i);
                continue;
            }

            // condition based purely on the angle of the surface
            // note: angle in settings is inverted to follow the same convention
            // as slicers such as cura
            let threshold = 90. - settings.criticality_settings.support_overhanging_angle;
            if angle_deg < threshold {
                to_return.push(i);
            }
        }
        to_return
    }
}
