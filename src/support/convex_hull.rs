use crate::{
    evolution::{Cost, Random},
    models::Point,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConvexHull {
    pub vertexes: Vec<Point>,
    pub triangles: Vec<[Point; 3]>,
    pub triangles_areas: Vec<f32>,
    pub total_area: f32,
    pub max_z: f32, // Cached max height for the sampling step
}

impl ConvexHull {
    pub fn new(mut points: Vec<Point>) -> ConvexHull {
        if points.len() <= 2 {
            let max_z = points.iter().map(|p| p.z).fold(0.0, f32::max);
            return ConvexHull {
                vertexes: points,
                triangles: Vec::new(),
                triangles_areas: Vec::new(),
                total_area: 0.0,
                max_z,
            };
        }

        // --- Your Graham Scan implementation (Calculated in XY plane) ---
        let (start_point_index, start_point) = points
            .iter()
            .copied()
            .enumerate()
            .min_by(|a, b| {
                a.1.y
                    .partial_cmp(&b.1.y)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| {
                        a.1.x
                            .partial_cmp(&b.1.x)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
            })
            .expect("there should always be a point");

        points.remove(start_point_index);
        points.sort_by_key(|x| {
            let diff = *x - start_point;
            let angle = f32::atan2(diff.y, diff.x);
            let distance = (diff.x * diff.x + diff.y * diff.y).sqrt();
            (Cost::new(-angle), Cost::new(distance))
        });

        let mut hull = vec![start_point];
        for &point in &points {
            while hull.len() > 1 {
                let len = hull.len();
                let v_prev = hull[len - 2] - hull[len - 1];
                let v_this = hull[len - 2] - point;
                let area = Point::cross_2d(v_prev, v_this);
                if area > 0.0 {
                    hull.pop();
                } else {
                    break;
                }
            }
            hull.push(point);
        }
        // -----------------------------------------------------------------

        // Track max height from the final hull vertices
        let mut max_z = 0.0_f32;
        let mut triangles = Vec::new();
        let mut triangles_areas = Vec::new();
        let mut total_area = 0.;

        if hull.len() >= 3 {
            let p0 = hull[0];
            max_z = max_z.max(p0.z);

            for i in 1..(hull.len() - 1) {
                let p1 = hull[i];
                let p2 = hull[i + 1];

                max_z = max_z.max(p1.z).max(p2.z);

                // Area calculations ignore Z completely
                let v1 = p1 - p0;
                let v2 = p2 - p0;
                let tri_area = (Point::cross_2d(v1, v2)).abs() * 0.5;

                if tri_area > 0.0 {
                    triangles.push([p0, p1, p2]);
                    triangles_areas.push(tri_area);
                    total_area += tri_area;
                }
            }
        } else {
            max_z = hull.iter().map(|p| p.z).fold(0.0, f32::max);
        }

        ConvexHull {
            vertexes: hull,
            triangles,
            triangles_areas,
            total_area,
            max_z,
        }
    }

    pub fn area(&self) -> f32 {
        // Since we compute and cache the area during construction,
        // we can return it instantly here!
        self.total_area
    }

    #[allow(dead_code)]
    pub fn perimeter(&self) -> f32 {
        let points = &self.vertexes;

        if points.len() < 2 {
            return 0.0;
        }

        let mut perimeter = 0.0;
        for i in 0..points.len() {
            let j = (i + 1) % points.len();
            let dx = points[j].x - points[i].x;
            let dy = points[j].y - points[i].y;
            perimeter += (dx * dx + dy * dy).sqrt();
        }

        perimeter
    }

    pub fn random_point(&self, rand: &Random) -> Point {
        if self.triangles.is_empty() || self.total_area <= 0.0 {
            return Point::ZERO;
        }
        let random_z = rand.next_f32(0.0, self.max_z);

        if self.triangles.is_empty() {
            let mut p =
                Point::random_in_between(self.random_point(rand), self.random_point(rand), rand);
            p.z = random_z;
            return p;
        }

        let t = rand
            .choose_weighted(&self.triangles, &self.triangles_areas)
            .expect("triangle selection can't fail");

        let r1 = rand.next_f32(0.0, 1.0);
        let r2 = rand.next_f32(0.0, 1.0);

        let sqrt_r1 = r1.sqrt();
        let w_a = 1.0 - sqrt_r1;
        let w_b = sqrt_r1 * (1.0 - r2);
        let w_c = sqrt_r1 * r2;

        Point {
            x: w_a * t[0].x + w_b * t[1].x + w_c * t[2].x,
            y: w_a * t[0].y + w_b * t[1].y + w_c * t[2].y,
            z: random_z,
        }
    }
}
