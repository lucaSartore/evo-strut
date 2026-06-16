use convexhull3d::Vertex;
use core::f32;
use nalgebra::{ArrayStorage, Const, Matrix, Matrix2, Vector3};
use serde::{Deserialize, Serialize};
use std::{
    hash::{Hash, Hasher},
    ops::{Add, Sub},
};

use crate::{
    evolution::Random, models::Settings, support::random_distribution::RandomDistribution,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Eq for Point {}
impl Hash for Point {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        fn hash_f32<H: Hasher>(val: f32, state: &mut H) {
            // Treat -0.0 and 0.0 as the same by forcing them to 0.0
            let val = if val == 0.0 { 0.0 } else { val };
            val.to_bits().hash(state);
        }

        hash_f32(self.x, state);
        hash_f32(self.y, state);
        hash_f32(self.z, state);
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Point {
    pub const ZERO: Point = Point {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    pub const UPWARD: Point = Point {
        x: 0.,
        y: 0.,
        z: 1.,
    };
    pub const DOWNWARD: Point = Point {
        x: 0.,
        y: 0.,
        z: -1.,
    };

    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x, y, z }
    }

    pub fn abs(&self) -> f32 {
        self.norm_sq().sqrt()
    }
    pub fn as_versor(&self) -> Point {
        let norm = self.abs();
        if norm == 0. {
            return Point::ZERO;
        }
        self.to_scaled(1. / norm)
    }

    pub fn dot(a: Point, b: Point) -> f32 {
        (a.x * b.x) + (a.y * b.y) + (a.z * b.z)
    }

    pub fn cross_2d(a: Point, b: Point) -> f32 {
        a.x * b.y - b.x * a.y
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
        let cos = Point::dot(a.as_versor(), b.as_versor()).clamp(-1., 1.);
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
        if distance == 0.0 {
            return vec![start];
        }

        let versor = (end - start).as_versor();
        let n_points = (distance / max_distance).ceil() as u32 + 1;

        let mut to_return = vec![];
        for i in 0..n_points {
            let scale = i as f32 / (n_points - 1) as f32;
            let p = start + versor.to_scaled(distance * scale);
            to_return.push(p);
        }
        to_return
    }

    /// return the angle formed by the vector start -> end
    /// on the horizon line. in radiants
    pub fn horizon_angle(start: Point, end: Point) -> f32 {
        let v = end - start;
        let v_horizon = Point {
            x: v.x,
            y: v.y,
            z: 0.,
        };
        Point::angle_between(&v, &v_horizon)
    }

    pub fn layer(&self, s: &Settings) -> usize {
        let layer_height = s.contact_points_optimization_settings.layer_height;
        (self.z / layer_height).ceil() as usize
    }

    pub fn is_facing_upward(&self) -> bool {
        Point::angle_between(self, &Point::UPWARD) <= std::f32::consts::PI / 2.
    }

    pub fn is_lower_or_equal_than(&self, other: &Point) -> bool {
        self.z <= other.z
    }

    pub fn random_in_between(a: Point, b: Point, rand: &Random) -> Point {
        let v = b - a;
        let scaler = rand.next_f32(0.0, 1.0);
        a + v.to_scaled(scaler)
    }

    pub fn random(mean: Point, std: f32, rand: &Random) -> Point {
        let distribution = RandomDistribution::Normal {
            mean: 0.,
            std_dev: std,
        };
        let x = mean.x + rand.next_distribution(&distribution);
        let y = mean.y + rand.next_distribution(&distribution);
        let z = mean.z + rand.next_distribution(&distribution);
        Point { x, y, z }
    }

    pub fn triangle_area(v1: Point, v2: Point, v3: Point) -> f32 {
        let a = v2 - v1;
        let b = v3 - v1;

        let c = Point::cross(a, b);

        c.abs() / 2.0
    }

    pub fn pyramid_area(v1: Point, v2: Point, v3: Point, v4: Point) -> f32 {
        let a = v1 - v4;
        let b = v2 - v4;
        let c = v3 - v4;
        Point::dot(a, Point::cross(b, c)).abs() / 6.0
    }

    /// create a new random point with z = 0 and x,y sampled from the a random distribution
    pub fn random_zero_z(mean: Point, covariance: &Matrix2<f32>, rand: &Random) -> Point {
        let distribution = RandomDistribution::Normal {
            mean: 0.,
            std_dev: 1.,
        };
        let u = rand.next_distribution(&distribution);
        let v = rand.next_distribution(&distribution);

        // Compute Cholesky decomposition of covariance matrix
        let l = covariance.cholesky().map(|x| x.l());

        // Apply linear transformation to independent standard normals
        let sample =
            l.map(|l| l * Matrix::<f32, Const<2>, Const<1>, _>::from_column_slice(&[u, v]));

        Point {
            x: mean.x + sample.map(|x| x[0]).unwrap_or(0.),
            y: mean.y + sample.map(|x| x[1]).unwrap_or(0.),
            z: 0.0,
        }
    }

    pub fn mean(points: &[Point]) -> Point {
        let sum = points.iter().fold(Point::ZERO, |acc, p| acc + *p);
        sum.to_scaled(1.0 / points.len() as f32)
    }

    pub fn norm_sq(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }
}

impl Into<[f32; 3]> for Point {
    fn into(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl Into<Vector3<f32>> for Point {
    fn into(self) -> Vector3<f32> {
        [self.x, self.y, self.z].into()
    }
}

impl From<Matrix<f32, Const<3>, Const<1>, ArrayStorage<f32, 3, 1>>> for Point {
    fn from(value: Matrix<f32, Const<3>, Const<1>, ArrayStorage<f32, 3, 1>>) -> Self {
        Point {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

impl From<Point> for rerun::Vec3D {
    fn from(value: Point) -> Self {
        rerun::Vec3D::new(value.x, value.y, value.z)
    }
}

impl From<Point> for convexhull3d::Vertex {
    fn from(value: Point) -> Self {
        Vertex::new(value.x as f64, value.y as f64, value.z as f64)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PointI {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl PointI {
    pub fn new(point: Point, divisor: f32) -> Self {
        Self {
            x: (point.x / divisor) as i32,
            y: (point.y / divisor) as i32,
            z: (point.z / divisor) as i32,
        }
    }

    pub fn to_float(&self, multiplier: f32) -> Point {
        Point {
            x: (self.x as f32 * multiplier),
            y: (self.y as f32 * multiplier),
            z: (self.z as f32 * multiplier),
        }
    }
}
