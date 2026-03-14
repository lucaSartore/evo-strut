use stl_io::Vector;
use std::ops::{Add, Sub};
use rerun::{Position3D, Vector3D};

#[derive(Debug, Default, Clone, Copy)]
pub struct Point{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Point {
    pub const UPWARD: Point = Point{x: 0., y: 0., z: 1.};
    pub const DOWNWARD: Point = Point{x: 0., y: 0., z: -1.};

    pub fn abs(&self) -> f32 {
        (
            self.x.powi(2) + 
            self.y.powi(2) + 
            self.z.powi(2)
        ).sqrt()
    }
    pub fn as_versor(&self) -> Point {
        let norm = self.abs();
        self.to_scaled(1./norm)
    }

    pub fn dot(a: Point, b: Point) -> f32 {
        (a.x * b.x) +
        (a.y * b.y) +
        (a.z * b.z)
    }

    pub fn cross(a: Point, b: Point) -> Point {
        Point {
            x: (a.y * b.z) - (a.z * b.y),
            y: (a.z * b.x) - (a.x * b.z),
            z: (a.x * b.y) - (a.y * b.x),
        }
    }

    /// return the angle between two versors (in radiants)
    pub fn angle_between(a: &Point, b: &Point) -> f32 {
        let cos = Point::dot(
            a.as_versor(),
            b.as_versor()
        );
        cos.acos()
    }

    pub fn scale(&mut self, factor: f32) {
        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
    }

    pub fn to_scaled(&self, factor: f32) -> Point {
        let mut new = self.clone();
        new.scale(factor);
        new
    }

    pub fn interpolate(start: Point, end: Point, max_distance: f32) -> Vec<Point> {
        let distance = (end - start).abs();
        let versor = (end - start).as_versor();
        let n_points = (distance / max_distance).ceil() as u32 + 1;

        let mut to_return = vec![];
        for i in 0..n_points {
            let scale = i as f32 / (n_points - 1) as f32;
            let p = start + versor.to_scaled(scale);
            to_return.push(p);
        }
        to_return
    }

}

impl From<Vector<f32>> for Point {
    fn from(value: Vector<f32>) -> Self {
        Point {
            x: value.0[0],
            y: value.0[1],
            z: value.0[2]
        }
    }
}
impl From<&Vector<f32>> for Point {
    fn from(value: &Vector<f32>) -> Self {
        Point {
            x: value.0[0],
            y: value.0[1],
            z: value.0[2]
        }
    }
}

impl From<Point> for Position3D {
    fn from(value: Point) -> Self {
        Position3D::new(value.x,value.y,value.z)
    }
}

impl From<Point> for Vector3D {
    fn from(value: Point) -> Self {
        [value.x,value.y,value.z].into()
    }
}
