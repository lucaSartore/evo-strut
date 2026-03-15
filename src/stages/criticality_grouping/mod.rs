use std::{collections::HashSet, marker::PhantomData};
use rayon::prelude::*;
use itertools::Itertools;

use crate::{
    models::{Point, Settings, SurfaceGraph, FaceId},
    stages::{CriticalityDetectedState, CriticalityGroupedState, Pipeline, PipelineBehaviourTrait},
};

pub struct CriticalityGroupingStage<TB>
where
    TB: PipelineBehaviourTrait,
{
    _d: PhantomData<TB>,
}

impl<TB> CriticalityGroupingStage<TB>
where
    TB: PipelineBehaviourTrait,
{
    pub fn execute(
        input: Pipeline<CriticalityDetectedState, TB>,
    ) -> Pipeline<CriticalityGroupedState, TB> {
        let graph = &input.state.graph;
        let settings = &input.state.settings;
        let critical = &input.state.critical;
        let grouped_areas = TB::TCriticalityGrouping::group_criticality(graph, settings, critical);
        Pipeline::from_state(CriticalityGroupedState {
            settings: input.state.settings,
            graph: input.state.graph,
            grouped_areas,
            critical: critical.iter().copied().collect()
        })
    }
}

/// trait that groups critical meshes into
pub trait CriticalityGrouper {
    fn group_criticality(
        graph: &SurfaceGraph,
        settings: &Settings,
        critical: &[FaceId],
    ) -> Vec<Vec<FaceId>>;
}

pub struct DistanceBasedCriticalityGrouper {}
impl DistanceBasedCriticalityGrouper {
    fn expand_critical(
        graph: &SurfaceGraph,
        max_distance: f32,
        critical: &[FaceId],
    ) -> Vec<FaceId>{
        let critical_set: HashSet<_> = critical.iter().copied().collect();
        let critical_in_border = Self::find_critical_border(graph, critical);

        let mut expanded_triangles = vec![];
        let mut to_visit: Vec<_> = critical_in_border.iter().copied().collect();
        let mut visited = HashSet::new();


        let border_points: Vec<_> = critical_in_border
            .iter()
            .map(|x| {
                graph.get_triangle(*x).center()
            })
            .collect();


        while let Some(node_id) = to_visit.pop() {
            if visited.contains(&node_id) {
                continue;
            }

            visited.insert(node_id);

            let node = graph.get_triangle(node_id);

            let is_close_enough = border_points
                .iter()
                .any(|p| {
                    let distance = (node.center() - *p).abs();
                    distance < max_distance
                });

            if !is_close_enough {
                continue
            }

            if !critical_set.contains(&node_id) {
                expanded_triangles.push(node_id);
            }

            // we don't want to expand into UPWARD facing surfaces
            if Point::angle_between(&node.normal(), &Point::UPWARD).to_degrees() <= 90. {
                continue
            }

            graph
                .iter_adjacent(node_id)
                .filter(|x| !critical_set.contains(&x.index))
                .for_each(|x| { to_visit.push(x.index) });
        }

        expanded_triangles
            .iter()
            .copied()
            .chain(critical.iter().copied())
            .collect()
    }

    /// return the critical triangles that are part
    /// of the border
    fn find_critical_border(
        graph: &SurfaceGraph,
        critical: &[FaceId],
    ) -> Vec<FaceId> {
        let critical_set: HashSet<_> = critical.iter().copied().collect();

        let mut critical_in_border = vec![];
        for c in critical {
            let is_in_border = graph
                .iter_adjacent(*c)
                .any(|x| !critical_set.contains(&x.index));
            if is_in_border {
                critical_in_border.push(*c);
            }
        }
        critical_in_border
    }

    /// group the set of critical polygons into sets where
    /// all the triangles are adjacent
    fn group_by_connected(
        graph: &SurfaceGraph,
        critical: &Vec<FaceId>,
    ) -> Vec<Vec<FaceId>> {
        let mut to_return = vec![];
        let critical_set: HashSet<_> = critical.iter().copied().collect();

        let mut visited = HashSet::new();

        for c in critical {
            if visited.contains(c) {
                continue;
            }
            let mut area = vec![];
            let mut to_visit = vec![*c];

            while let Some(e) = to_visit.pop() {
                visited.insert(e);
                area.push(e);

                graph.iter_adjacent(e)
                    .filter(|x| {!visited.contains(&x.index) && critical_set.contains(&x.index)})
                    .for_each(|x| {to_visit.push(x.index);});
            }

            to_return.push(area);
        }

        to_return
    }
    
}

impl CriticalityGrouper for DistanceBasedCriticalityGrouper {
    fn group_criticality(
        graph: &SurfaceGraph,
        settings: &Settings,
        critical: &[FaceId],
    ) -> Vec<Vec<FaceId>> {

        let expanded = Self::expand_critical(graph, settings.criticality_settings.criticality_expansion_rate, critical);
        Self::group_by_connected(graph, &expanded) 
    }
}
