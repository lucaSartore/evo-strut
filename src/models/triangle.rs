use rerun::TriangleIndices;
use stl_io::IndexedTriangle;
use super::{Point, SurfaceGraph};

pub struct Triangle<'a> {
    pub graph: &'a SurfaceGraph,
    pub index: usize
}

impl Triangle<'_> {
    pub fn vertex_a(&self) -> Point{
        let t = self.as_raw_indexed();
        self.graph.get_point(t.vertices[0])
    }
    pub fn vertex_b(&self) -> Point{
        let t = self.as_raw_indexed();
        self.graph.get_point(t.vertices[0])
    }
    pub fn vertex_c(&self) -> Point{
        let t = self.as_raw_indexed();
        self.graph.get_point(t.vertices[0])
    }
    pub fn normal(&self) -> Point{
        let t = self.as_raw_indexed();
        t.normal.into()
    }
    pub fn as_raw_indexed(&self) -> IndexedTriangle{
        self.graph.mesh.faces[self.index]
    }
}


impl<'a> From<Triangle<'a>> for TriangleIndices {
    fn from(value: Triangle) -> Self {
        let v = value.graph.mesh.faces[value.index].vertices;
        [
            v[0] as u32,
            v[1] as u32,
            v[2] as u32
        ].into()
    }
}
