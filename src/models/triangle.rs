use rerun::TriangleIndices;
use stl_io::IndexedTriangle;
use crate::models::TriangleId;

use super::{Point, SurfaceGraph};

pub struct Triangle<'a> {
    pub graph: &'a SurfaceGraph,
    pub index: TriangleId
}

impl Triangle<'_> {
    pub fn center(&self) -> Point{
        let [a ,b ,c ] = self.vertexes();
        let mut p = a + b + c;
        p.scale(1./3.);
        p
    }
    pub fn vertexes(&self) -> [Point;3]{
        let t = self.as_raw_indexed();
        [
            self.graph.get_point(t.vertices[0].into()),
            self.graph.get_point(t.vertices[1].into()),
            self.graph.get_point(t.vertices[2].into())
        ]
    }
    pub fn vertex_a(&self) -> Point{
        let t = self.as_raw_indexed();
        self.graph.get_point(t.vertices[0].into())
    }
    pub fn vertex_b(&self) -> Point{
        let t = self.as_raw_indexed();
        self.graph.get_point(t.vertices[0].into())
    }
    pub fn vertex_c(&self) -> Point{
        let t = self.as_raw_indexed();
        self.graph.get_point(t.vertices[0].into())
    }
    pub fn normal(&self) -> Point{
        let t = self.as_raw_indexed();
        t.normal.into()
    }
    pub fn as_raw_indexed(&self) -> IndexedTriangle{
        self.graph.mesh.faces[self.index.0]
    }

    pub  fn get_height_difference(&self, other: &Triangle<'_>) -> f32 {
        let distance = self.center() - other.center();
        distance.z
    }

    pub  fn is_lower_than(&self, other: &Triangle<'_>) -> bool {
        self.get_height_difference(other) < 0.
    }
    pub  fn is_lower_or_equal_than(&self, other: &Triangle<'_>) -> bool {
        self.get_height_difference(other) <= 0.
    }
    pub  fn is_higher_than(&self, other: &Triangle<'_>) -> bool {
        self.get_height_difference(other) > 0.
    }
    pub  fn is_higher_or_equal_than(&self, other: &Triangle<'_>) -> bool {
        self.get_height_difference(other) >= 0.
    }
}


impl<'a> From<Triangle<'a>> for TriangleIndices {
    fn from(value: Triangle) -> Self {
        let v = value.graph.mesh.faces[value.index.0].vertices;
        [
            v[0] as u32,
            v[1] as u32,
            v[2] as u32
        ].into()
    }
}
