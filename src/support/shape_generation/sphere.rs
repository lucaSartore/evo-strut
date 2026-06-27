use anyhow::{anyhow, Result};
use baby_shark::{io::Builder, mesh::corner_table::CornerTableF, voxel::volume::Volume};

use crate::{
    models::Point,
    support::shape_generation::{builder_wrapper::BuilderWrapper, to_volumes, ShapeGenerator},
};

#[derive(Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f32,
}
impl Sphere {
    pub fn new(center: Point, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl ShapeGenerator for Sphere {
    fn build(&self, voxel_size: f32) -> Result<Volume> {
        if self.radius <= 0. {
            return Err(anyhow!("Sphere radius must be positive"));
        }
        if voxel_size <= 0. {
            return Err(anyhow!("voxel size must be positive"));
        }

        let mut builder = BuilderWrapper::new(CornerTableF::builder_indexed(), self.center);
        let latitude_segments = ((std::f32::consts::PI * self.radius) / voxel_size)
            .ceil()
            .max(2.) as usize;
        let longitude_segments = ((2. * std::f32::consts::PI * self.radius) / voxel_size)
            .ceil()
            .max(3.) as usize;

        let top = builder.add_vertex(self.center + Point::UPWARD.to_scaled(self.radius))?;
        let bottom = builder.add_vertex(self.center + Point::DOWNWARD.to_scaled(self.radius))?;

        let mut rings = Vec::with_capacity(latitude_segments - 1);
        for latitude in 1..latitude_segments {
            let theta = std::f32::consts::PI * latitude as f32 / latitude_segments as f32;
            let ring_radius = theta.sin() * self.radius;
            let z = theta.cos() * self.radius;
            let mut ring = Vec::with_capacity(longitude_segments);

            for longitude in 0..longitude_segments {
                let phi = 2. * std::f32::consts::PI * longitude as f32 / longitude_segments as f32;
                let point =
                    self.center + Point::new(ring_radius * phi.cos(), ring_radius * phi.sin(), z);
                ring.push(builder.add_vertex(point)?);
            }

            rings.push(ring);
        }

        for longitude in 0..longitude_segments {
            let next_longitude = (longitude + 1) % longitude_segments;
            builder.add_face(top, rings[0][longitude], rings[0][next_longitude])?;
        }

        for latitude in 0..rings.len() - 1 {
            for longitude in 0..longitude_segments {
                let next_longitude = (longitude + 1) % longitude_segments;
                let current = &rings[latitude];
                let next = &rings[latitude + 1];

                builder.add_face(current[longitude], next[longitude], next[next_longitude])?;
                builder.add_face(
                    current[longitude],
                    next[next_longitude],
                    current[next_longitude],
                )?;
            }
        }

        let last_ring = &rings[rings.len() - 1];
        for longitude in 0..longitude_segments {
            let next_longitude = (longitude + 1) % longitude_segments;
            builder.add_face(last_ring[longitude], bottom, last_ring[next_longitude])?;
        }

        let mesh = builder.finish()?;

        to_volumes(&mesh, voxel_size)
    }
}
