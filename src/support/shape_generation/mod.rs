use std::fmt::Debug;

use baby_shark::io::{Builder};
use anyhow::{Result, anyhow};


mod truncated_cone;
pub use truncated_cone::{Circle, TruncatedCone};
mod sphere;
pub use sphere::Sphere;
#[cfg(test)]
pub mod test;



pub struct ShapeFactory<TMesh>
    where TMesh: Builder<Mesh = TMesh, Scalar = f32> {
    /// shapes that will be added to the shape
    positive_shapes: Vec<Box<dyn ShapeGenerator<TMesh>>>,
    /// shapes that will be subtracted to the final shape
    negative_shapes: Vec<Box<dyn ShapeGenerator<TMesh>>>
}

impl<TMesh> ShapeFactory<TMesh> 
    where TMesh: Builder<Mesh = TMesh, Scalar = f32>
{
    pub fn new() -> Self {
        Self {
            positive_shapes: vec![],
            negative_shapes: vec![]
        }
    }

    pub fn add_positive_shape<T>(&mut self, shape: T)
        where T : ShapeGenerator<TMesh> + 'static
    {
        self.positive_shapes.push(Box::new(shape));
    }

    pub fn add_negative_shape<T>(&mut self, shape: T)
        where T : ShapeGenerator<TMesh> + 'static
    {
        self.positive_shapes.push(Box::new(shape));
    }

    pub fn build(&self) {
    }
}

pub trait ShapeGenerator<TMesh>
    where TMesh: Builder<Mesh = TMesh, Scalar = f32>
{
    fn build(&self, vertex_size: f32) -> Result<TMesh>;
}

fn map_err<TV, TE>(value: std::result::Result<TV, TE>) -> Result<TV>
    where TE: Debug
{
    value.map_err(|x| anyhow!("Error in mesh creation: {x:?}"))
}

