use std::collections::HashMap;
use smallvec::{self, SmallVec};
use super::*;


pub struct GridNeighbor {
    pub id: (i32, i32),
    // if the neighbors is lower than the current element
    pub is_lower: bool,
    // if the neighbors is critical or not
    pub is_critical: bool
}

pub struct GridElement {
    /// the id of the triangle that generated the element
    pub id: TriangleId,
    /// the point
    pub point: Point,
    /// the angle between a downward facing vector and the surface inclination
    pub angle: f32,
    /// the set of neighbors
    pub neighbors: SmallVec<[GridNeighbor; 4]>
}

impl GridElement {
    pub fn iter_neighbors(& self, filter_lower: Option<bool>, filter_critical: Option<bool>) -> impl Iterator<Item = &GridNeighbor> {
        self
            .neighbors
            .iter()
            .filter(move |x| filter_lower.is_none() || Some(x.is_lower) == filter_lower)
            .filter(move |x| filter_critical.is_none() || Some(x.is_critical) == filter_critical)
    }
}

// a transformation of a triangle-based surface into a
// same-size rectangle based one a grid of sampled points
pub struct SurfaceGrid {
    pub discretization_size: f32,
    pub points: HashMap<(i32, i32), GridElement>,
    pub bucketed_triangles: BucketedTriangles
}

impl SurfaceGrid {
    pub fn new(
        graph: &SurfaceGraph,
        area: &[TriangleId],
        critical: & HashSet<TriangleId>,
        discretization_size: f32
    ) -> Self {
        let bt = BucketedTriangles::new(discretization_size, graph, area);
        let mut to_return = Self{
            discretization_size,
            points: HashMap::new(),
            bucketed_triangles: bt
        };

        to_return.generate_points(graph);
        to_return.generate_neighbors(critical);
        to_return
    }

    fn generate_points(&mut self, graph: &SurfaceGraph) {
        for c in self.bucketed_triangles.iter_coordinates() {
            let mut point = identifier_to_zero_point(self.discretization_size, *c);
            let t_id = self.bucketed_triangles.find_triangle_that_includes_approximated(graph, point)
                .expect("triangle should always be found here");

            let t = graph.get_triangle(t_id);
            point.z = t.find_z(point.x, point.y);

            // calculating the angle of the surface w.r.t. the vector facing downward
            let angle = Point::angle_between(&Point::DOWNWARD, &t.normal());
            let angle_deg = angle.to_degrees();

            self.points.insert(*c, GridElement { 
                id: t_id,
                point,
                angle: angle_deg,
                neighbors: SmallVec::new()
            });
        }

    }
    fn generate_neighbors(&mut self, critical: & HashSet<TriangleId>) {
        for id in self.bucketed_triangles.iter_coordinates().copied() {
            let height_this = self.points.get(&id).expect("id shall always be present").point.z;
            let x = id.0;
            let y = id.1;
            let adjacent = vec! [
                (x+1, y),
                (x-1, y),
                (1, y+1),
                (1, y-1),
            ];
            for adj in adjacent {
                let mut neighbor = None;
                if let Some(p) = self.points.get(&adj) {
                    neighbor = GridNeighbor{
                        id: adj,
                        is_lower: p.point.z < height_this,
                        is_critical: critical.contains(&p.id)
                    }.into();
                }
                if let Some(n) = neighbor {
                    self
                        .points
                        .get_mut(&id)
                        .expect("id shall always be present")
                        .neighbors
                        .push(n);
                }
            }
        }
    }
}
