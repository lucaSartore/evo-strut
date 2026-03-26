use crate::{models::{FaceId, MeshId, SurfaceGraph}, stages::{ContactPointsDecidedState, CriticalityDetectedState, CriticalityGroupedState, LoadedState, Pipeline, PipelineBehaviourTrait, PipelineState}};
use anyhow::Result;

mod color;
pub use color::Color;


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
            for v in t.vertexes {
                colors[v.0 as usize] = Color::Red;
            }
        });

        visualize_mesh(graph, "detected critical surfaces", Some(colors))
    }
}


impl Visualizer<CriticalityGroupedState> for VisualizationStage {
    fn visualize<TB: PipelineBehaviourTrait>(pipeline: &Pipeline<CriticalityGroupedState, TB>) -> Result<()> {
        let graph = &pipeline.state.graph;

        let mut colors = vec![Color::White; graph.count_vertices()];

        let critical_group = &pipeline.state.grouped_areas;

        for (i, group) in critical_group.iter().enumerate() {
            let hue = i as f32 * 360. / critical_group.len() as f32;
            let color = Color::Hsv(hue, 1.0, 1.0);
            for triangle_id in group {
                let t = graph.get_triangle(*triangle_id).as_raw_indexed();
                for v in t.vertexes {
                    colors[v.0 as usize] = color;
                }
            }
        }

        visualize_mesh(graph, "grouped critical surfaces", Some(colors))
    }
}


impl Visualizer<ContactPointsDecidedState> for VisualizationStage {
    fn visualize<TB: PipelineBehaviourTrait>(pipeline: &Pipeline<ContactPointsDecidedState, TB>) -> Result<()> {
        let graph = &pipeline.state.graph;

        let mut colors = vec![Color::Green; graph.count_vertices()];

        pipeline
            .state
            .critical
            .iter()
            .enumerate()
            .filter(|(_,x)| **x)
            .for_each(|(id,_)| {
                pipeline
                    .state
                    .graph
                    .get_triangle(FaceId(id as u32))
                    .vertexes_index()
                    .iter()
                    .for_each(|v| colors[v.index()] = Color::Red);
            });

        let rec = rerun::RecordingStreamBuilder::new("decided contact points").spawn()?;


        rec.log(
            "mesh",
            &rerun::Mesh3D::new(graph.iter_vertices())
                .with_vertex_normals(graph.vertex_normals(None))
                .with_vertex_colors(colors)
                .with_triangle_indices(graph.iter_triangles(None)),
        )?;

        let cp = pipeline
            .state
            .connection_points
            .iter_contacts()
            .map(|x| graph.get_triangle(*x.0).center());

        rec.log(
            "support_points", 
            &rerun::Points3D::new(cp)
        )?;

        Ok(())
    }
}
fn visualize_mesh(graph: &SurfaceGraph, name: &str, colors: Option<Vec<Color>>) -> Result<()> {
    let rec = rerun::RecordingStreamBuilder::new(name).spawn()?;

    let mut colors = match colors {
        Some(e) => e,
        None => vec![Color::Green; graph.count_vertices()]
    };

    rec.log(
        "vertexes", 
        &rerun::Points3D::new(graph.iter_vertices())
    )?;

    rec.log(
        name,
        &rerun::Mesh3D::new(graph.iter_vertices())
            .with_vertex_normals(graph.vertex_normals(None))
            .with_vertex_colors(colors)
            .with_triangle_indices(graph.iter_triangles(None)),
    )?;

    Ok(())
}
