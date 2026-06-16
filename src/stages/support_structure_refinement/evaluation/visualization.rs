use crate::{
    models::{Point, SurfaceGraph},
    stages::{
        support_structure_refinement::{
            SupportNode, SupportStructureGene, evaluation::logic::genome_to_graph_descriptor,
        },
        visualization::Color,
    },
};
use anyhow::Result;
use rerun::RecordingStream;

pub fn visualize(
    rec: &RecordingStream,
    gene: &SupportStructureGene,
    mesh: &SurfaceGraph,
) -> Result<()> {
    let descriptor = genome_to_graph_descriptor(gene);
    let colors = vec![Color::Green; mesh.count_vertices()];

    rec.log(
        "mesh",
        &rerun::Mesh3D::new(mesh.iter_vertices())
            .with_vertex_normals(mesh.vertex_normals(None))
            .with_vertex_colors(colors)
            .with_triangle_indices(mesh.iter_triangles(None)),
    )?;

    let lines: Vec<_> = descriptor
        .edges
        .iter()
        .flat_map(|(id, neighbors)| {
            let p = descriptor.details[id].position;
            neighbors
                .iter()
                .filter(|x| **x < *id)
                .map(|x| [p, descriptor.details[x].position])
                .collect::<Vec<_>>()
        })
        .collect();

    rec.log("support_structure", &rerun::LineStrips3D::new(lines))?;

    let cones: Vec<[Point; 2]> = gene
        .nodes
        .iter()
        .flat_map(|(_, x)| {
            if let SupportNode::Contact(n) = x {
                Some(n.leans_on.iter().map(|support| {
                    let sp = descriptor.details[support].position;
                    [
                        [sp, n.position + Point::new(0., n.radius, 0.)],
                        [sp, n.position + Point::new(0., -n.radius, 0.)],
                        [sp, n.position + Point::new(n.radius, 0., 0.)],
                        [sp, n.position + Point::new(-n.radius, 0., 0.)],
                    ]
                }))
            } else {
                None
            }
        })
        .flatten()
        .flatten()
        .collect();

    rec.log("support_cones", &rerun::LineStrips3D::new(cones))?;

    Ok(())
}
