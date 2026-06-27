use crate::{
    models::{Point, Settings, SurfaceGraph},
    stages::{
        support_structure_optimization::SupportStructureOptimizationGene, visualization::Color,
    },
};
use anyhow::Result;
use rerun::RecordingStream;

pub fn visualize(
    rec: &RecordingStream,
    gene: &SupportStructureOptimizationGene,
    mesh: &SurfaceGraph,
    settings: &Settings,
) -> Result<()> {
    let descriptor = gene.to_graph_descriptor(mesh, settings);
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

    let cones: Vec<[Point; 2]> = descriptor
        .details
        .values()
        .flat_map(|x| {
            if x.is_contact {
                let values = descriptor.edges[&x.id].iter().map(|neighbor_id| {
                    let sp = descriptor.details[neighbor_id].position;
                    [
                        [sp, x.position + Point::new(0., x.radius, 0.)],
                        [sp, x.position + Point::new(0., -x.radius, -1.)],
                        [sp, x.position + Point::new(x.radius, 0., 0.)],
                        [sp, x.position + Point::new(-x.radius, 0., 0.)],
                    ]
                });
                Some(values)
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
