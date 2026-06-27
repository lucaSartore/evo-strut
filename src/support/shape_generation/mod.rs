use anyhow::{anyhow, Result};
use baby_shark::{
    io::Builder,
    mesh::{
        corner_table::CornerTableF, polygon_soup::data_structure::PolygonSoup, traits::Triangles,
    },
    voxel::{
        meshing::ActiveVoxelsMesher,
        prelude::{DualContouringMesher, MarchingCubesMesher, MeshToVolume},
        volume::Volume,
    },
};

mod truncated_cone;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
pub use truncated_cone::{Circle, TruncatedCone};
mod sphere;
pub use sphere::Sphere;
mod builder_wrapper;

use crate::models::{Settings, SupportSettings};

pub struct ShapeFactory {
    /// shapes that will be added to the shape
    positive_shapes: Vec<Box<dyn ShapeGenerator + Sync>>,
    positive_volumes: Vec<Volume>,
    /// shapes that will be subtracted to the final shape
    negative_shapes: Vec<Box<dyn ShapeGenerator + Sync>>,
    negative_volumes: Vec<Volume>,
}

impl ShapeFactory {
    pub fn new() -> Self {
        Self {
            positive_shapes: vec![],
            negative_shapes: vec![],
            positive_volumes: vec![],
            negative_volumes: vec![],
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
        T: ShapeGenerator + 'static + Sync,
    {
        self.positive_shapes.push(Box::new(shape));
    }

    pub fn add_negative_shape<T>(&mut self, shape: T)
    where
        T: ShapeGenerator + 'static + Sync,
    {
        self.negative_shapes.push(Box::new(shape));
    }

    pub fn build(self, settings: &Settings) -> Result<CornerTableF> {
        let s = &settings.support_settings;

        let all_positive = self
            .positive_shapes
            .par_iter()
            .flat_map(|x| x.build(s.voxel_size))
            .chain(self.positive_volumes)
            .reduce(
                || Volume::with_voxel_size(s.voxel_size),
                |v1, v2| v1.union(v2),
            );

        let all_negative = self
            .negative_shapes
            .par_iter()
            .flat_map(|x| x.build(s.voxel_size))
            .chain(self.negative_volumes)
            .reduce(
                || Volume::with_voxel_size(s.voxel_size),
                |v1, v2| v1.union(v2),
            );

        let final_volume = all_positive.subtract(all_negative);

        let vertices = MarchingCubesMesher::default()
            .with_voxel_size(s.voxel_size)
            .mesh(&final_volume);

        // let vertices = DualContouringMesher::default()
        //     .with_voxel_size(s.voxel_size)
        //     .mesh(&final_volume).unwrap();

        let soup = PolygonSoup::from_vertices(vertices);

        Ok(CornerTableF::from_vertex_and_face_iters(
            soup.vertices().copied(),
            0..soup.vertices().count(),
        ))
    }
}

pub fn to_volumes(mesh: &CornerTableF, voxel_size: f32) -> Result<Volume> {
    let mut converter = MeshToVolume::default().with_voxel_size(voxel_size);
    let volume = converter.convert(mesh).ok_or(anyhow!("conversion fail"))?;
    Ok(volume)
}

pub trait ShapeGenerator {
    fn build(&self, vertex_size: f32) -> Result<Volume>;
}
