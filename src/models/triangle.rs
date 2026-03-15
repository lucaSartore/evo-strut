use log::warn;
use rerun::TriangleIndices;
use crate::models::{PointId, FaceId};
use super::*;

pub struct Triangle<'a> {
    pub graph: &'a SurfaceGraph,
    pub index: FaceId
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
            self.graph.get_point(t.vertexes[0].into()),
            self.graph.get_point(t.vertexes[1].into()),
            self.graph.get_point(t.vertexes[2].into())
        ]
    }
    pub fn vertexes_index(&self) -> [PointId;3]{
        let t = self.as_raw_indexed();
        [
            t.vertexes[0].into(),
            t.vertexes[1].into(),
            t.vertexes[2].into(),
        ]
    }
    pub fn vertex_a(&self) -> Point{
        let t = self.as_raw_indexed();
        self.graph.get_point(t.vertexes[0].into())
    }
    pub fn vertex_b(&self) -> Point{
        let t = self.as_raw_indexed();
        self.graph.get_point(t.vertexes[1].into())
    }
    pub fn vertex_c(&self) -> Point{
        let t = self.as_raw_indexed();
        self.graph.get_point(t.vertexes[2].into())
    }
    pub fn normal(&self) -> Point{
        let t = self.as_raw_indexed();
        t.normal.into()
    }
    pub fn as_raw_indexed(&self) -> Face{
        self.graph.mesh.faces[self.index]
    }

    pub fn get_height_difference(&self, other: &Triangle<'_>) -> f32 {
        let distance = self.center() - other.center();
        distance.z
    }

    pub fn is_lower_than(&self, other: &Triangle<'_>) -> bool {
        self.get_height_difference(other) < 0.
    }
    pub fn is_lower_or_equal_than(&self, other: &Triangle<'_>) -> bool {
        self.get_height_difference(other) <= 0.
    }
    pub fn is_higher_than(&self, other: &Triangle<'_>) -> bool {
        self.get_height_difference(other) > 0.
    }
    pub fn is_higher_or_equal_than(&self, other: &Triangle<'_>) -> bool {
        self.get_height_difference(other) >= 0.
    }

    /// calculate the area of the triangle using the cross product
    pub fn area(&self) -> f32 {
        let [v1, v2, v3] = self.vertexes();

        let a = v2 - v1;
        let b = v3 - v1;

        let c = Point::cross(a,b);

        c.abs() / 2.0
    }

    // find out if a point is inside the footprint (i.e. the projection
    // on the x/y plane) of the triangle
    pub fn is_point_inside_footprint(&self, point: Point) -> bool {
        let [v1, v2, v3] = self.vertexes();
        // 2D cross product of vectors
        let sign = |p1: Point, p2: Point, p3: Point| -> f32 {
            (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y)
        };

        let d1 = sign(point, v1, v2);
        let d2 = sign(point, v2, v3);
        let d3 = sign(point, v3, v1);

        let has_neg = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
        let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);

        // The point is inside if all cross products have the same sign
        // (i.e., it's not "outside" any of the three edges).
        !(has_neg && has_pos)
    }

    pub fn find_z(&self, x: f32, y: f32) -> f32 {

        let [v1, v2, v3] = self.vertexes();

        // two vectors on the triangle's plane
        let a = v2 - v1;
        let b = v3 - v1;

        // normal vector (identify a plane)
        let n = Point::cross(a, b);

        // The triangle is vertical; the line X=x, Y=y might not intersect 
        if n.z.abs() < f32::EPSILON {
            warn!("trying to find the z coordinate of a vertical rectangle");
            // fallback value
            return v1.z;
        }

        // intersection between the line X=x, Y=y with the plane identified
        // by the current triangle
        v1.z - (n.x * (x - v1.x) + n.y * (y - v1.y)) / n.z
    }
}


impl<'a> From<Triangle<'a>> for TriangleIndices {
    fn from(value: Triangle) -> Self {
        let v = value.graph.mesh.faces[value.index].vertexes;
        [
            v[0].0,
            v[1].0,
            v[2].0
        ].into()
    }
}
