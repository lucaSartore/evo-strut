use anyhow::anyhow;
use baby_shark::io::{Builder, IndexedBuilder};

use crate::{models::Point, support::shape_generation::{ShapeGenerator, builder_wrapper::BuilderWrapper}};

pub struct Circle {
    pub center: Point,
    pub radius: f32,
    /// versor representing the orientation of the circle.
    /// the versor is perpendcular to the plane that contains the circle
    pub orientation: Point,
}
impl Circle {
    pub fn new(center: Point, radius: f32, orientation: Point) -> Self {
        Self {
            center,
            radius,
            orientation,
        }
    }
}
pub struct TruncatedCone {
    pub bottom: Circle,
    pub top: Circle,
}
impl TruncatedCone {
    pub fn new(bottom: Circle, top: Circle) -> Self {
        Self { bottom, top }
    }
}

fn circle_basis(orientation: Point) -> anyhow::Result<(Point, Point)> {
    let normal = orientation.as_versor();
    if normal == Point::ZERO {
        return Err(anyhow!("Circle orientation must be non-zero"));
    }

    let reference = if normal.z.abs() < 0.9 {
        Point::UPWARD
    } else {
        Point::new(1., 0., 0.)
    };
    let u = Point::cross(reference, normal).as_versor();
    let v = Point::cross(normal, u).as_versor();

    Ok((u, v))
}

fn circle_segments(radius: f32, vertex_size: f32) -> usize {
    ((2. * std::f32::consts::PI * radius) / vertex_size)
        .ceil()
        .max(3.) as usize
}

fn circle_points(circle: &Circle, segments: usize) -> anyhow::Result<Vec<Point>> {
    let (u, v) = circle_basis(circle.orientation)?;
    let mut points = Vec::with_capacity(segments);

    for segment in 0..segments {
        let angle = 2. * std::f32::consts::PI * segment as f32 / segments as f32;
        points.push(
            circle.center
                + u.to_scaled(circle.radius * angle.cos())
                + v.to_scaled(circle.radius * angle.sin()),
        );
    }

    Ok(points)
}


impl<TMesh> ShapeGenerator<TMesh> for TruncatedCone
where
    TMesh: Builder<Mesh = TMesh, Scalar = f32>,
{
    fn build(&self, vertex_size: f32) -> anyhow::Result<TMesh> {
        if self.bottom.radius <= 0. || self.top.radius <= 0. {
            return Err(anyhow!("Truncated cone radii must be positive"));
        }
        if vertex_size <= 0. {
            return Err(anyhow!("Vertex size must be positive"));
        }

        let segments = circle_segments(self.bottom.radius.max(self.top.radius), vertex_size);
        let bottom_points = circle_points(&self.bottom, segments)?;
        let top_points = circle_points(&self.top, segments)?;
        let axial_segments = (bottom_points
            .iter()
            .zip(top_points.iter())
            .map(|(bottom, top)| (*top - *bottom).abs())
            .fold(0., f32::max)
            / vertex_size)
            .ceil()
            .max(1.) as usize;

        let center = (self.bottom.center + self.top.center).to_scaled(0.5);
        let mut builder = BuilderWrapper::new(TMesh::builder_indexed(), center);
        let bottom_center = builder.add_vertex(self.bottom.center)?;
        let top_center = builder.add_vertex(self.top.center)?;
        let mut rings = Vec::with_capacity(axial_segments + 1);

        for axial_segment in 0..=axial_segments {
            let t = axial_segment as f32 / axial_segments as f32;
            let mut ring = Vec::with_capacity(segments);

            for segment in 0..segments {
                let point = bottom_points[segment]
                    + (top_points[segment] - bottom_points[segment]).to_scaled(t);
                ring.push(builder.add_vertex(point)?);
            }

            rings.push(ring);
        }

        let axis = self.top.center - self.bottom.center;
        let bottom_outward = if Point::dot(self.bottom.orientation, axis) > 0. {
            false
        } else {
            true
        };
        let top_outward = Point::dot(self.top.orientation, axis) > 0.;

        for segment in 0..segments {
            let next_segment = (segment + 1) % segments;
            if bottom_outward {
                builder.add_face(
                    bottom_center,
                    rings[0][segment],
                    rings[0][next_segment],
                )?;
            } else {
                builder.add_face(
                    bottom_center,
                    rings[0][next_segment],
                    rings[0][segment],
                )?;
            }

            if top_outward {
                builder.add_face(
                    top_center,
                    rings[axial_segments][segment],
                    rings[axial_segments][next_segment],
                )?;
            } else {
                builder.add_face(
                    top_center,
                    rings[axial_segments][next_segment],
                    rings[axial_segments][segment],
                )?;
            }
        }

        for axial_segment in 0..axial_segments {
            for segment in 0..segments {
                let next_segment = (segment + 1) % segments;
                let current = &rings[axial_segment];
                let next = &rings[axial_segment + 1];

                builder.add_face(current[segment], next[next_segment], next[segment])?;
                builder.add_face(
                    current[segment],
                    current[next_segment],
                    next[next_segment],
                )?;
            }
        }

        builder.finish()
    }
}
