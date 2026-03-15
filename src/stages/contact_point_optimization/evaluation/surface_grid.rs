use std::collections::HashSet;
use smallvec::smallvec;

use crate::models::Triangle;

use super::*;


pub type Coordinates = (i32, i32);

pub struct SurfacePoint {
    pub point: Point,
    pub normal: Point,
    pub neighbors: SmallVec<[Coordinates; 4]>,
    pub critical: bool,
}

impl SurfacePoint {
    pub fn new(point: Point, normal: Point, critical: bool) -> Self {
        Self {
            point,
            normal,
            critical,
            neighbors: smallvec![]
        }
    }

    /// merge the most critical aspects of self and another element
    pub fn pick_most_critical(&mut self, other: &Self) {
        if other.point.z < self.point.z {
            self.point.z = other.point.z;
        }
        if other.normal.z < self.normal.z {
            self.normal = other.normal
        }
        self.critical |= other.critical;
    }
}

pub struct SurfaceGrid {
    pub discretization_size: f32,
    pub points: HashMap<Coordinates, SurfacePoint>
}

impl SurfaceGrid {
    pub fn discretize_to_point(&self, identifier: Coordinates) -> Point {
        let x = identifier.0 as f32 * self.discretization_size;
        let y = identifier.1 as f32 * self.discretization_size;
        Point{ x, y, z: 0. }
    }
    pub fn point_to_discretized(&self, point: Point) -> Coordinates {
        let x = (point.x / self.discretization_size).round() as i32;
        let y = (point.y / self.discretization_size).round() as i32;
        (x,y)
    }
    pub fn new(graph: &SurfaceGraph, critical: &HashSet<FaceId>, area: &[FaceId], discretization_size: f32) -> Self {
        let mut to_return = Self{
            discretization_size,
            points: HashMap::default()
        };

        for t_id in area.iter() {
            let t = graph.get_triangle(*t_id);
            let is_critical = critical.contains(t_id);
            let points = to_return.all_points_of_triangle(&t);

            for point in points {
                // intersection point between the approximated coordinate
                let mut discretized_point = to_return.discretize_to_point(point);
                discretized_point.z = t.find_z(discretized_point.x, discretized_point.y);
                let normal = t.normal();

                let sp = SurfacePoint::new(discretized_point, normal, is_critical);
                if let Some(e) = to_return.points.get_mut(&point) {
                    e.pick_most_critical(&sp);
                } else {
                    to_return.points.insert(point, sp);
                }
            }
        }
        to_return.insert_neighbors();
        to_return
    }

    fn insert_neighbors(&mut self) {
        let coordinates: Vec<_> = self.points.keys().copied().collect();

        for (x,y) in coordinates.into_iter() {
            for adj in [
                (x+1, y),
                (x-1, y),
                (1, y+1),
                (1, y-1),
            ] {
                if self.points.contains_key(&adj) {
                    self
                        .points
                        .get_mut(&(x,y))
                        .expect("point shall be present here")
                        .neighbors.push(adj);
                }
            }
        }
    }

    // return all the points of a triangle
    fn all_points_of_triangle (&self, t: &Triangle<'_>) -> Vec<Coordinates>{
        let [v1, v2, v3] = t.vertexes();
        let s1 = Point::interpolate(v1, v2, self.discretization_size).into_iter();
        let s2 = Point::interpolate(v2, v3, self.discretization_size).into_iter();
        let s3 = Point::interpolate(v3, v1, self.discretization_size).into_iter();

        s1
            .chain(s2)
            .chain(s3)
            .map(|x| self.point_to_discretized(x))
            .chunk_by(|x| x.0) // order by x
            .into_iter()
            .flat_map(|(x,chunk)| {
                let (y_min, y_max) = chunk
                    // select y component
                    .map(|e| e.1)
                    // find min and max
                    .fold((i32::MAX, i32::MIN), |(min, max), y| {
                        (
                            i32::min(min, y),
                            i32::max(max, y)
                        )
                    });
                (y_min..=y_max).map(move |y| (x,y)) // all y coordinates
            })
            .unique()
            .collect()
    }
}
