use std::marker::PhantomData;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use smallvec::{SmallVec, smallvec};

use crate::{evolution::Cost, models::{FaceId, Point, PointId, Settings, SurfaceGraph, Triangle}, stages::{CriticalityDetectedState, LoadedState, Pipeline, PipelineBehaviourTrait, criticality_detection::propagation::KnownCosts }};

pub mod propagation;
use propagation::PropagationEvaluator;


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
    fn detect_criticality(graph: &SurfaceGraph, settings: &Settings) -> Vec<FaceId>;
}
fn is_triangle_close_to_the_ground(triangle: &Triangle<'_>, settings: &Settings) -> bool {
    triangle.center().z <= settings.criticality_settings.max_detachment_from_z_plane
}

pub struct OrientationBasedCriticalityDetector {}
impl CriticalityDetector for OrientationBasedCriticalityDetector {
    fn detect_criticality(graph: &SurfaceGraph, settings: &Settings) -> Vec<FaceId> {
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


pub struct PropagationBasedCriticalityDetector {}

impl PropagationBasedCriticalityDetector {
    // find all the concavities.
    // Concavities are automatically supported because they have the mesh
    // below, however they are indistinguishable from a critical point
    // if not for the surface normals. So we need to find them in order to
    // calculate their criticality properly
    fn find_concavity(graph: &SurfaceGraph) -> HashSet<FaceId> {
        let mut point_to_triangles: HashMap<PointId, SmallVec<[Triangle<'_>; 4]>> = Default::default();

        for t in graph.iter_triangles(None) {
            for p in t.vertexes_index() {
                let vec = point_to_triangles.entry(p).or_insert(smallvec![]);
                vec.push(t);
            }
        }

        // conditions for a concavity:
        //  1) all triangles are facing upward
        //  2) the point is the lowest among all of hist neighbor
        let points: HashSet<_> = point_to_triangles
            .iter()
            .filter(|(_,triangles)|{
                triangles
                    .iter()
                    .all(|t| t.normal().is_facing_upward())
            })
            .filter(|(point_id,triangles)|{
                let point = graph.get_point(**point_id);
                triangles
                    .iter()
                    .map(|x| x.vertexes())
                    .flatten()
                    .unique()
                    .all(|new_p| point.is_lower_or_equal_than(&new_p))
            })
            .map(|(p,_)| *p)
            .collect();

        point_to_triangles
            .iter()
            .filter(|(p,_)| points.contains(*p))
            .map(|(_,t)| t)
            .flatten()
            .map(|t| t.index)
            .collect()
    }
}

struct HeightBasedKnownCost<'a> {
    graph: &'a SurfaceGraph,
    settings: &'a Settings
}
impl<'a> KnownCosts for HeightBasedKnownCost<'a> {
    fn cost_of(&self, id: FaceId) -> Option<Cost> {
        let t = self.graph.get_triangle(id);
        if is_triangle_close_to_the_ground(&t, self.settings) {
            return Some(Cost::ZERO);
        }
        None
    }
}

impl CriticalityDetector for PropagationBasedCriticalityDetector {
    fn detect_criticality(graph: &SurfaceGraph, settings: &Settings) -> Vec<FaceId> {
        let known_costs: HashMap<FaceId, Cost> = graph
            .iter_triangles(None)
            .filter(|x| is_triangle_close_to_the_ground(x, settings))
            .map(|x| (x.index, Cost::ZERO))
            .collect();

        let area: Vec<_> = graph.iter_triangles(None)
            .map(|x| x.index)
            .filter(|x| !known_costs.contains_key(x))
            .collect();

        let pm =  PropagationEvaluator::new(
            graph,
            settings,
            &area, 
            HeightBasedKnownCost{ graph, settings }
        );

        let supported = Self::find_concavity(graph);
        let costs = pm.evaluate(&|x| supported.contains(&x));

        costs
            .iter()
            .filter(|(_,c)| c.unit_cost != Cost::ZERO)
            .map(|(k,_)| *k)
            .collect()
    }
}

