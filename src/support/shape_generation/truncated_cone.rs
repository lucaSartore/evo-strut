use anyhow::{anyhow, Result};
use baby_shark::io::Builder;
use rerun::external::glam::usize;

use crate::{evolution::Cost, models::Point, support::shape_generation::{ShapeGenerator, builder_wrapper::BuilderWrapper}};

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

fn circle_basis(orientation: Point) -> Result<(Point, Point)> {
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

fn circle_points(circle: &Circle, segments: usize) -> Result<Vec<Point>> {
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


fn find_closest(point: Point, options: &[Point], l1: Point, l2: Point) -> usize {
    let reference_vector = perpendicular_unit_vector(l1, l2, point);
    options
        .iter()
        .enumerate()
        .max_by_key(|(_, p)| {
            let vector = perpendicular_unit_vector(l1, l2, **p);
            let similarity = Point::dot(vector, reference_vector);
            Cost::new(similarity)
        })
        .expect("there shall be at least a point")
        .0
}

fn perpendicular_unit_vector(l1: Point, l2: Point, p: Point) -> Point {
    // converting from point-point to point-versor
    let v = (l2 - l1).as_versor();
    let q = l1;

    let m = Point::dot(p - q, v);

    // x is the point on the line that together with p forms
    // a line perpendicular to l1-l2
    let x = q + v.to_scaled(m);

    (p - x).as_versor()
}

impl<TMesh> ShapeGenerator<TMesh> for TruncatedCone
where
    TMesh: Builder<Mesh = TMesh, Scalar = f32>,
{
    fn build(&self, vertex_size: f32) -> Result<TMesh> {
        if self.bottom.radius <= 0. || self.top.radius <= 0. {
            return Err(anyhow!("Truncated cone radii must be positive"));
        }
        if vertex_size <= 0. {
            return Err(anyhow!("Vertex size must be positive"));
        }

        // creating the top circles
        let segments = circle_segments(self.bottom.radius, vertex_size);
        let bottom_points = circle_points(&self.bottom, segments)?;
        let segments = circle_segments(self.top.radius, vertex_size);
        let top_points = circle_points(&self.top, segments)?;

        // creating the builder
        let center = (self.bottom.center + self.top.center).to_scaled(0.5);
        let mut builder = BuilderWrapper::new(TMesh::builder_indexed(), center);

        // adding the points
        let bottom_center = builder.add_vertex(self.bottom.center)?;
        let top_center = builder.add_vertex(self.top.center)?;
        let bottom_points_ids = bottom_points.iter().map(|p| builder.add_vertex(*p)).collect::<Result<Vec<usize>>>()?;
        let top_points_ids = top_points.iter().map(|p| builder.add_vertex(*p)).collect::<Result<Vec<usize>>>()?;

        for i in 0..bottom_points.len() {
            let i_next = (i + 1) % bottom_points.len();
            let id1 = bottom_points_ids[i];
            let id2 = bottom_points_ids[i_next];
            builder.add_face(id1, id2, bottom_center)?
        }

        for i in 0..top_points.len() {
            let i_next = (i + 1) % top_points.len();
            let id1 = top_points_ids[i];
            let id2 = top_points_ids[i_next];
            builder.add_face(id1, id2, top_center)?
        }

        let mut skipped_ranges = vec![];
        let mut closest_prev = find_closest(
            *bottom_points.last().expect("ponts can't be empty"),
            &top_points,
            self.top.center,
            self.bottom.center
        );
        for (i, point) in bottom_points.iter().enumerate() {
            let next = (i + 1) % bottom_points.len();
            let closest = find_closest(*point, &top_points, self.top.center, self.bottom.center);
            builder.add_face(
                bottom_points_ids[i],
                bottom_points_ids[next],
                top_points_ids[closest]
            )?;

            if closest_prev != closest {
                skipped_ranges.push((closest_prev, closest, i));
                    closest_prev = closest;
            }
        }

        for (from, to, source) in skipped_ranges {
            let mut i = from;
            loop {
                let next = (i + 1) % top_points.len();

                builder.add_face(
                    top_points_ids[i],
                    top_points_ids[next],
                    bottom_points_ids[source]
                )?;

                if next == to {
                    break;
                }
                i = next;
            }
        }

        builder.finish()
    }
}
