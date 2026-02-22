use crate::{models::{Point, Settings, SurfaceGraph}, stages::criticality_detection::{CriticalityDetector, OrientationBasedCriticality}};
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

    let mut colors = vec![color; mesh.count_vertices()];

    rec.log(
        "mesh",
        &rerun::Mesh3D::new(mesh.iter_vertices())
            .with_vertex_normals(mesh.vertex_normals())
            .with_vertex_colors(colors)
            .with_triangle_indices(mesh.iter_triangles()),
    )?;

    Ok(())
}


pub fn visualize_critical_surfaces(mesh: &SurfaceGraph) -> Result<()> {

    let rec = rerun::RecordingStreamBuilder::new("Critical surfaces").spawn()?;


    let mut colors = vec![Color::Green; mesh.count_vertices()];

    let critical_surfaces_indexes = OrientationBasedCriticality::detect_criticality(
        mesh,
        &Settings::default()
    );

    critical_surfaces_indexes.iter().for_each(|x| {
        let t = mesh.get_triangle(*x).as_raw_indexed();
        for v in t.vertices {
            colors[v] = Color::Red;
        }
    });


    rec.log(
        "mesh",
        &rerun::Mesh3D::new(mesh.iter_vertices())
            .with_vertex_normals(mesh.vertex_normals())
            .with_vertex_colors(colors)
            .with_triangle_indices(mesh.iter_triangles()),
    )?;

    Ok(())
}
