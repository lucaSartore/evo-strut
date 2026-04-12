use crate::{models::{Point, SurfaceGraph}, stages::{support_structure_optimization::{ContactNode, SupportNode, SupportStructureGene, evaluation::logic::genome_to_graph_descriptor}, visualization::Color}};
use anyhow::Result;
use itertools::Position;

pub fn visualize(gene: &SupportStructureGene, mesh: & SurfaceGraph) -> Result<()> {
    let rec = rerun::RecordingStreamBuilder::new("contact points structure optimization").spawn()?;

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
            let p = descriptor.positions[id];
            neighbors
                .iter()
                .filter(|x| **x < *id)
                .map(|x| [p, descriptor.positions[x]])
                .collect::<Vec<_>>()
        }).collect();

    println!("{:?}",descriptor.edges);
    println!("{:?}",lines);
    rec.log(
        "support_structure",
        &rerun::LineStrips3D::new(lines)
    )?;

    let cones: Vec<[Point;2]> = gene
        .nodes
        .iter()
        .flat_map(|(_,x)| {
            if let SupportNode::Contact(n) = x {
                Some(n
                    .leans_on
                    .iter()
                    .map(|support| {
                        let sp = descriptor.positions[support];
                        [
                            [sp, n.position + Point::new(0., n.radius, 0.)],
                            [sp, n.position + Point::new(0., -n.radius, 0.)],
                            [sp, n.position + Point::new(n.radius, 0., 0.)],
                            [sp, n.position + Point::new(-n.radius, 0., 0.)]
                        ]
                    }))
            } else {
                None
            }
        })
        .flatten()
        .flatten()
        .collect();
        

    rec.log(
        "support_cones",
        &rerun::LineStrips3D::new(cones)
    )?;

    Ok(())
}
