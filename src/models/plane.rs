use super::*;

#[derive(Clone, Debug)]
pub struct Plane {
    pub normal: Point,
    pub d: f32,
}

impl Plane {
    /// Generates a plane from three points
    pub fn from_points(p1: Point, p2: Point, p3: Point) -> Self {
        let v1 = p2 - p1;
        let v2 = p3 - p1;

        let normal = Point::cross(v1, v2);

        let d = -Point::dot(normal, p1);

        Plane { normal, d }
    }

    /// Generate a plane that includes p1 and p2, and maximizes the distance from p_max_distance
    pub fn from_points_and_max_distance(p1: Point, p2: Point, p_max_distance: Point) -> Self {
        let v = Plane::from_points(p1, p2, p_max_distance).normal;
        let p3 = p1 + v;
        return Plane::from_points(p1, p2, p3);
    }

    /// classify if a point is in one side or the other of
    /// the plane (using the normal to distinguish
    pub fn classify_point(&self, p: Point) -> bool {
        Point::dot(self.normal, p) + self.d > 0.0
    }
}
