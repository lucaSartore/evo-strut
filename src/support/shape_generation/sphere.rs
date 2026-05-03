use baby_shark::io::{Builder, IndexedBuilder};
use super::map_err;

use crate::{models::Point, support::shape_generation::ShapeGenerator};


pub struct Sphere {
    pub center: Point,
    pub radius: f32
}
impl Sphere {
    pub fn new(center: Point, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl<TMesh> ShapeGenerator<TMesh> for Sphere
    where TMesh: Builder<Mesh = TMesh, Scalar = f32> 
{
    fn build(&self, vertex_size: f32) -> anyhow::Result<TMesh> {
        let mut builder = TMesh::builder_indexed();
        let i1 = map_err(builder.add_vertex(Point::new(0., 0., 0.)))?;
        let i2 = map_err(builder.add_vertex(Point::new(1., 0., 0.)))?;
        let i3 = map_err(builder.add_vertex(Point::new(0., 1., 0.)))?;
        map_err(builder.add_face(i1, i2, i3))?;
        map_err(builder.finish())
    }
}



