use crate::models::{SurfaceGraph, Point};
use anyhow::Result;

use rerun::{
    TriangleIndices, Vector3D, blueprint, components::Color as RerunColor, demo_util::{bounce_lerp, color_spiral}, external::crossbeam::epoch::Pointable
};


#[derive(Copy, Clone, Debug)]
pub enum Color {
    Blue,
    Red,
    Green,
    RGB(u8, u8, u8)
}

impl From<Color> for RerunColor {
    fn from(value: Color) -> Self {
        match value {
            Color::Blue => RerunColor::from_rgb(0,0,255),
            Color::Green => RerunColor::from_rgb(0,255,0),
            Color::Red => RerunColor::from_rgb(255,0,0),
            Color::RGB(r, g, b) => RerunColor::from_rgb(r, g, b)
        }
    }
}

pub fn visualization_test() {
    const NUM_POINTS: usize = 100;
    const TAU: f32 = 0.5;

    let rec = rerun::RecordingStreamBuilder::new("rerun_example_dna_abacus")
        .connect_grpc().unwrap();

    let (points1, colors1) = color_spiral(NUM_POINTS, 2.0, 0.02, 0.0, 0.1);
    let (points2, colors2) = color_spiral(NUM_POINTS, 2.0, 0.02, TAU * 0.5, 0.1);

    rec.log(
        "dna/structure/left",
        &rerun::Points3D::new(points1.iter().copied())
            .with_colors(colors1)
            .with_radii([0.08]),
    ).unwrap();
    rec.log(
        "dna/structure/right",
        &rerun::Points3D::new(points2.iter().copied())
            .with_colors(colors2)
            .with_radii([0.08]),
    ).unwrap();
}


pub  fn visualize_mesh(mesh: SurfaceGraph, name: &str, color: Color) -> Result<()> {

    let rec = rerun::RecordingStreamBuilder::new(name).spawn()?;

    let points = mesh
        .mesh
        .vertices
        .iter()
        .map(|x| {
            let y: Point = x.into();
            y
        });


    let triangles = mesh
        .nodes
        .iter()
        .map(|x| {
            let v = mesh.mesh.faces[x.triangle].vertices;
            let y: TriangleIndices = [v[0] as u32,v[1] as u32,v[2] as u32].into();
            y
        });

    let mut normals = vec![[0.,0.,0.]; mesh.mesh.vertices.len()];

    mesh.mesh
        .faces
        .iter()
        .for_each(|x| {
            let normal = x.normal;
            for v in x.vertices {
                normals[v][0] = normal[0];
                normals[v][1] = normal[1];
                normals[v][2] = normal[2];
            }
        });

    let colors = (0..mesh.mesh.vertices.len())
        .map(|_| { color });
    rec.log(
        "mesh",
        &rerun::Mesh3D::new(points)
            .with_vertex_normals(normals)
            .with_vertex_colors(colors)
            .with_triangle_indices(triangles),
    )?;

    Ok(())
}
