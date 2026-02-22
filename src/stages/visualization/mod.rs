use crate::models::{SurfaceGraph, Point};
use anyhow::Result;

use rerun::{
    TriangleIndices,
    components::Color as RerunColor
};


#[derive(Copy, Clone, Debug)]
pub enum Color {
    Blue,
    Red,
    Green,
    Rgb(u8, u8, u8)
}

impl From<Color> for RerunColor {
    fn from(value: Color) -> Self {
        match value {
            Color::Blue => RerunColor::from_rgb(0,0,255),
            Color::Green => RerunColor::from_rgb(0,255,0),
            Color::Red => RerunColor::from_rgb(255,0,0),
            Color::Rgb(r, g, b) => RerunColor::from_rgb(r, g, b)
        }
    }
}


pub fn visualize_mesh(mesh: SurfaceGraph, name: &str, color: Color) -> Result<()> {

    let rec = rerun::RecordingStreamBuilder::new(name).spawn()?;

    let colors = (0..mesh.mesh.vertices.len())
        .map(|_| { color });

    rec.log(
        "mesh",
        &rerun::Mesh3D::new(mesh.iter_vertices())
            .with_vertex_normals(mesh.vertex_normals())
            .with_vertex_colors(colors)
            .with_triangle_indices(mesh.iter_triangles()),
    )?;

    Ok(())
}
