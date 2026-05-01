use crate::{evolution::Cost, models::Point};

#[derive(Clone, Debug)]
pub struct ConvexHull {
    pub vertexes: Vec<Point>
}

impl ConvexHull {
    pub fn new(mut points: Vec<Point>) -> ConvexHull {
        if points.len() <= 2 {
            return ConvexHull { vertexes: points };
        }

        let (start_point_index, start_point) = points
            .iter()
            .copied()
            .enumerate()
            .min_by(|a, b| {
                a.1.y.partial_cmp(&b.1.y).unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| a.1.x.partial_cmp(&b.1.x).unwrap_or(std::cmp::Ordering::Equal))
            })
            .expect("there should always be a point");

        points.remove(start_point_index);
        points.sort_by_key(|x| {
            let diff = *x - start_point;
            let angle = f32::atan2(diff.y, diff.x);
            let distance = diff.abs();
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

        ConvexHull { vertexes: hull }
    }

    pub fn area(&self) -> f32 {
        let points = &self.vertexes;
        
        if points.len() < 3 {
            return 0.0;
        }

        let mut area = 0.0;
        for i in 0..points.len() {
            let j = (i + 1) % points.len();
            area += points[i].x * points[j].y;
            area -= points[j].x * points[i].y;
        }

        area.abs() / 2.0
    }

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_convex_hull() {
        let points = vec![
            Point { x: 0.0, y: 0.0, z: 0.0 },
            Point { x: 4.0, y: 0.0, z: 0.0 },
            Point { x: 2.0, y: 3.0, z: 0.0 },
        ];
        let hull = ConvexHull::new(points);
        assert_eq!(hull.vertexes.len(), 3);
    }

    #[test]
    fn test_square_convex_hull() {
        let points = vec![
            Point { x: 0.0, y: 0.0, z: 0.0 },
            Point { x: 1.0, y: 0.0, z: 0.0 },
            Point { x: 1.0, y: 1.0, z: 0.0 },
            Point { x: 0.0, y: 1.0, z: 0.0 },
            Point { x: 0.5, y: 0.5, z: 0.0 }, // Interior point
        ];
        let hull = ConvexHull::new(points);
        assert_eq!(hull.vertexes.len(), 4);
    }

    #[test]
    fn test_convex_hull_ignores_z() {
        let points = vec![
            Point { x: 0.0, y: 0.0, z: 100.0 },
            Point { x: 1.0, y: 0.0, z: -50.0 },
            Point { x: 0.5, y: 1.0, z: 0.0 },
        ];
        let hull = ConvexHull::new(points);
        assert_eq!(hull.vertexes.len(), 3);
    }

    #[test]
    fn test_single_point_hull() {
        let points = vec![Point { x: 5.0, y: 5.0, z: 0.0 }];
        let hull = ConvexHull::new(points);
        assert_eq!(hull.vertexes.len(), 1);
    }

    #[test]
    fn test_two_points_hull() {
        let points = vec![
            Point { x: 0.0, y: 0.0, z: 0.0 },
            Point { x: 1.0, y: 1.0, z: 0.0 },
        ];
        let hull = ConvexHull::new(points);
        assert_eq!(hull.vertexes.len(), 2);
    }

    #[test]
    fn test_triangle_area() {
        let points = vec![
            Point { x: 0.0, y: 0.0, z: 0.0 },
            Point { x: 4.0, y: 0.0, z: 0.0 },
            Point { x: 0.0, y: 3.0, z: 0.0 },
        ];
        let hull = ConvexHull::new(points);
        let area = hull.area();
        assert!((area - 6.0).abs() < 0.0001);
    }

    #[test]
    fn test_square_area() {
        let points = vec![
            Point { x: 0.0, y: 0.0, z: 0.0 },
            Point { x: 2.0, y: 0.0, z: 0.0 },
            Point { x: 2.0, y: 2.0, z: 0.0 },
            Point { x: 0.0, y: 2.0, z: 0.0 },
            Point { x: 1.0, y: 1.0, z: 1.0 },
            Point { x: 1.1, y: 0.3, z: 1.0 },
        ];
        let hull = ConvexHull::new(points);
        let area = hull.area();
        assert!((area - 4.0).abs() < 0.0001);
    }

    #[test]
    fn test_empty_shape_area() {
        let shape = ConvexHull { vertexes: vec![] };
        let area = shape.area();
        assert_eq!(area, 0.0);
    }

    #[test]
    fn test_single_point_area() {
        let shape = ConvexHull {
            vertexes: vec![Point { x: 1.0, y: 1.0, z: 0.0 }],
        };
        let area = shape.area();
        assert_eq!(area, 0.0);
    }

    #[test]
    fn test_two_point_area() {
        let shape = ConvexHull {
            vertexes: vec![
                Point { x: 0.0, y: 0.0, z: 0.0 },
                Point { x: 1.0, y: 1.0, z: 0.0 },
            ],
        };
        let area = shape.area();
        assert_eq!(area, 0.0);
    }

    #[test]
    fn test_pentagon_area() {
        let points = vec![
            Point { x: 1.0, y: 0.0, z: 0.0 },
            Point { x: 0.309, y: 0.951, z: 0.0 },
            Point { x: -0.809, y: 0.588, z: 0.0 },
            Point { x: -0.809, y: -0.588, z: 0.0 },
            Point { x: 0.309, y: -0.951, z: 0.0 },
        ];
        let hull = ConvexHull::new(points);
        let area = hull.area();
        assert!(area > 0.0 && area < 3.0);
    }

    #[test]
    fn test_square_perimeter() {
        let points = vec![
            Point { x: 0.0, y: 0.0, z: 0.0 },
            Point { x: 1.0, y: 0.0, z: 0.0 },
            Point { x: 1.0, y: 1.0, z: 0.0 },
            Point { x: 0.0, y: 1.0, z: 0.0 },
        ];
        let hull = ConvexHull::new(points);
        let perimeter = hull.perimeter();
        assert!((perimeter - 4.0).abs() < 0.0001);
    }

    #[test]
    fn test_triangle_perimeter() {
        let points = vec![
            Point { x: 0.0, y: 0.0, z: 0.0 },
            Point { x: 3.0, y: 0.0, z: 0.0 },
            Point { x: 0.0, y: 4.0, z: 0.0 },
        ];
        let hull = ConvexHull::new(points);
        let perimeter = hull.perimeter();
        // 3 + 4 + 5 = 12
        assert!((perimeter - 12.0).abs() < 0.0001);
    }

    #[test]
    fn test_empty_perimeter() {
        let shape = ConvexHull { vertexes: vec![] };
        let perimeter = shape.perimeter();
        assert_eq!(perimeter, 0.0);
    }

    #[test]
    fn test_single_point_perimeter() {
        let shape = ConvexHull {
            vertexes: vec![Point { x: 1.0, y: 1.0, z: 0.0 }],
        };
        let perimeter = shape.perimeter();
        assert_eq!(perimeter, 0.0);
    }

    #[test]
    fn test_two_point_perimeter() {
        let shape = ConvexHull {
            vertexes: vec![
                Point { x: 0.0, y: 0.0, z: 0.0 },
                Point { x: 3.0, y: 4.0, z: 0.0 },
            ],
        };
        let perimeter = shape.perimeter();
        // Distance from (0,0) to (3,4) is 5, so perimeter is 5 + 5 = 10
        assert!((perimeter - 10.0).abs() < 0.0001);
    }
}
