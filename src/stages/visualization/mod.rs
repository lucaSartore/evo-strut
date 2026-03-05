use crate::{models::{Point, Settings, SurfaceGraph}, stages::{CriticalityDetectedState, LoadedState, Pipeline, PipelineBehaviourTrait, PipelineState, criticality_detection::{CriticalityDetector, OrientationBasedCriticalityDetector}}};
use anyhow::Result;

mod color;
pub use color::Color;

use rerun::{
    TriangleIndices,
    components::Color as RerunColor
};

pub trait Visualizer<TS>
where 
    TS: PipelineState
{
    fn visualize<TB: PipelineBehaviourTrait>(pipeline: &Pipeline<TS, TB>) -> Result<()>;
}

pub struct VisualizationStage {}

impl Visualizer<LoadedState> for VisualizationStage {
    /// visualize the loaded model in with a simple mesh
    fn visualize<TB: PipelineBehaviourTrait>(pipeline: &Pipeline<LoadedState, TB>) -> Result<()> {
        visualize_mesh(&pipeline.state.graph, "loaded model", None)
    }
}

impl Visualizer<CriticalityDetectedState> for VisualizationStage {
    fn visualize<TB: PipelineBehaviourTrait>(pipeline: &Pipeline<CriticalityDetectedState, TB>) -> Result<()> {
        let graph = &pipeline.state.graph;

        let mut colors = vec![Color::Green; graph.count_vertices()];

        let critical_surfaces = &pipeline.state.critical;

        critical_surfaces.iter().for_each(|x| {
            let t = graph.get_triangle(*x).as_raw_indexed();
            for v in t.vertices {
                colors[v] = Color::Red;
            }
        });

        visualize_mesh(graph, "detected critical surfaces", Some(colors))
    }
}


fn visualize_mesh(graph: &SurfaceGraph, name: &str, colors: Option<Vec<Color>>) -> Result<()> {
    let rec = rerun::RecordingStreamBuilder::new(name).spawn()?;

    let colors = match colors {
        Some(e) => e,
        None => vec![Color::Green; graph.count_vertices()]
    };

    rec.log(
        name,
        &rerun::Mesh3D::new(graph.iter_vertices())
            .with_vertex_normals(graph.vertex_normals())
            .with_vertex_colors(colors)
            .with_triangle_indices(graph.iter_triangles()),
    )?;

    Ok(())
}
