use crate::models::Point;

#[derive(Clone, Debug)]
pub struct ConvexHull3D {
    pub hull: Option<convexhull3d::ConvexHull3D>,
    pub points: Vec<Point>,
    pub center: Point,
}

impl ConvexHull3D {
    pub fn new(points: Vec<Point>) -> Self {
        let vertexes: Vec<convexhull3d::Vertex> = points.iter().copied().map(Into::into).collect();
        let Ok(hull) = convexhull3d::ConvexHull3D::build(&vertexes) else {
            let center = points.iter().fold(Point::ZERO, |acc, p| acc + *p);
            return Self {
                hull: None,
                points,
                center
            };
        };
        let center = points
            .iter()
            .copied()
            .reduce(|p1, p2| p1 + p2)
            .expect("reduce can't fail if point's length is >= 3")
            .to_scaled(1.0 / points.len() as f32);
        Self {
            hull: hull.into(),
            points,
            center,
        }
    }

    fn iter_faces(&self) -> Option<impl Iterator<Item = [Point; 3]>> {
        self.hull
            .as_ref()?
            .faces()
            .iter()
            .map(|face| {
                let [p1_i, p2_i, p3_i] = face.indices();
                let p1 = self.points[p1_i];
                let p2 = self.points[p2_i];
                let p3 = self.points[p3_i];
                [p1, p2, p3]
            })
            .into()
    }

    pub fn area(&self) -> f32 {
        let mut area = 0.0;
        let Some(faces) = self.iter_faces() else {
            // approximation for areas with minimum amount of points
            for p in self.points.iter() {
                let v = (*p - self.center).abs();
                area += v.powi(2);
            }
            return area;
        };
        for [v1, v2, v3] in faces {
            area += Point::triangle_area(v1, v2, v3);
        }
        area
    }

    pub fn volume(&self) -> f32 {
        let mut volume = 0.0;
        let Some(faces) = self.iter_faces() else {
            return 0.0;
        };
        for [v1, v2, v3] in faces {
            volume += Point::pyramid_area(v1, v2, v3, self.center);
        }
        volume
    }
}
