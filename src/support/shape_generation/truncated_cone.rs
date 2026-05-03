use baby_shark::io::Builder;

use crate::{models::Point, support::shape_generation::ShapeGenerator};


pub struct Circle {
    pub center: Point,
    pub radius: f32,
    /// versor representing the orientation of the circle.
    /// the versor is perpendcular to the plane that contains the circle
    pub orientation: Point
}
pub  struct TruncatedCone {
    pub bottom: Circle,
    pub top: Circle
}


impl<TMesh> ShapeGenerator<TMesh> for Circle
    where TMesh: Builder<Mesh = TMesh, Scalar = f32> {
    fn build(&self, vertex_size: f32) -> anyhow::Result<TMesh> 
        where TMesh: baby_shark::io::Builder<Mesh = TMesh> {
        todo!()
    }
}
