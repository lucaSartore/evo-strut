use stl_io::Vector;
use std::ops::{Add, Sub};
use rerun::{Position3D, Vector3D};

#[derive(Debug, Default, Clone)]
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
    pub fn abs(&self) -> f32 {
        (
            self.x.powi(2) + 
            self.y.powi(2) + 
            self.z.powi(2)
        ).sqrt()
    }
    pub fn as_versor(&self) -> Point {
        let norm = self.abs();
        Point { 
            x: self.x/norm,
            y: self.y/norm,
            z: self.z/norm 
        }
    }
    pub fn dot(a: Point, b: Point) -> f32 {
        (a.x - b.x).powi(2) +
        (a.y - b.y).powi(2) +
        (a.z - b.z).powi(2)
    }

    pub fn angle_between(a: Point, b: Point) -> f32 {
        let cos = Point::dot(
            a.as_versor(),
            b.as_versor()
        );
        cos.acos()
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
