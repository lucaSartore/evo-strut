
use anyhow::{anyhow, Result};
use baby_shark::{io::Builder, mesh::{corner_table::CornerTableF, polygon_soup::data_structure::PolygonSoup, traits::Triangles}, voxel::{prelude::{MarchingCubesMesher, MeshToVolume}, volume::Volume}};

mod truncated_cone;
pub use truncated_cone::{Circle, TruncatedCone};
mod sphere;
pub use sphere::Sphere;
mod builder_wrapper;

use crate::models::{Settings, SupportSettings};
#[cfg(test)]
pub mod test;

pub struct ShapeFactory
{
    /// shapes that will be added to the shape
    positive_shapes: Vec<Box<dyn ShapeGenerator<CornerTableF>>>,
    positive_volumes: Vec<Volume>,
    /// shapes that will be subtracted to the final shape
    negative_shapes: Vec<Box<dyn ShapeGenerator<CornerTableF>>>,
    negative_volumes: Vec<Volume>,
}

impl ShapeFactory
{
    pub fn new() -> Self {
        Self {
            positive_shapes: vec![],
            negative_shapes: vec![],
            positive_volumes: vec![],
            negative_volumes: vec![]
        }
    }

    pub fn add_positive_volume(&mut self, volume: Volume) {
        self.positive_volumes.push(volume);
    }
    pub fn add_negative_volume(&mut self, volume: Volume) {
        self.negative_volumes.push(volume);
    }
    pub fn add_positive_shape<T>(&mut self, shape: T)
    where
        T: ShapeGenerator<CornerTableF> + 'static,
    {
        self.positive_shapes.push(Box::new(shape));
    }

    pub fn add_negative_shape<T>(&mut self, shape: T)
    where
        T: ShapeGenerator<CornerTableF> + 'static,
    {
        self.negative_shapes.push(Box::new(shape));
    }

    pub fn build(&self, settings: &Settings) -> Result<CornerTableF> {
        let s = &settings.support_settings;
        let mut positive = Self::to_volumes(&self.positive_shapes, s)?;
        positive.append(&mut self.positive_volumes.clone());

        let mut negative = Self::to_volumes(&self.negative_shapes, s)?;
        negative.append(&mut self.negative_volumes.clone());

        let all_positive = positive
            .into_iter()
            .reduce(|v1, v2| v1.union(v2))
            .ok_or(anyhow!("at least one positive shape should be provided"))?;

        // let final_volume = negative
        //     .into_iter()
        //     .fold(all_positive, |acc, v| acc.subtract(v));

        let vertices = MarchingCubesMesher::default()
            .with_voxel_size(s.merging_voxel_size)
            .mesh(&all_positive);

        let soup = PolygonSoup::from_vertices(vertices);

        Ok(CornerTableF::from_vertex_and_face_iters(
            soup.vertices().copied(),
            0..soup.vertices().count()
        ))
    }

    fn to_volumes(shapes: &[Box<dyn ShapeGenerator<CornerTableF>>], settings: &SupportSettings) -> Result<Vec<Volume>> {
        let mut to_return = vec![];

        for shape in shapes {
            let mut converter = MeshToVolume::default().with_voxel_size(settings.merging_voxel_size);
            let mesh = shape.build(settings.primitive_voxel_size)?;
            let volume = converter.convert(&mesh).ok_or(anyhow!("conversion fail"))?;
            to_return.push(volume);
        }
        Ok(to_return)
    }
}

pub trait ShapeGenerator<TMesh>
where
    TMesh: Builder<Mesh = TMesh, Scalar = f32>
{
    fn build(&self, vertex_size: f32) -> Result<TMesh>;
}

