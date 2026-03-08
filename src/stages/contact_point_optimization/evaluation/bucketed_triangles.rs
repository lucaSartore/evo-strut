use std::collections::{HashMap, HashSet};
use crate::{models::{ Point, Settings, SurfaceGraph, TriangleId}, stages::contact_point_optimization::models::ContactPointsGene};
use itertools::Itertools;

use super::*;

/// put a triangle-based grit into a set of buckets
/// so that querying which triangle contains a point is easy
pub struct BucketedTriangles {
    discretization_size: f32,
    discrete_position_to_triangles: HashMap<(i32, i32), Vec<TriangleId>>,
}

impl BucketedTriangles {
    pub fn new(discretization_size: f32, graph: &SurfaceGraph, area_to_evaluate: &[TriangleId]) -> Self {

        let mut map: HashMap<(i32, i32), Vec<TriangleId>> = HashMap::new();
        // for each triangle
        for t in area_to_evaluate {
            let triangle = graph.get_triangle(*t);
            let mut elements = vec![];
            let [v1, v2, v3] = triangle.vertexes();

            // we sample points along the vertexes (with a size that make shure
            // se place a point into all "bukets"
            let side1 = Point::interpolate(v1, v2, discretization_size * 0.4);
            let side2 = Point::interpolate(v2, v3, discretization_size * 0.4);
            let side3 = Point::interpolate(v3, v1, discretization_size * 0.4);

            // generating an identifier for each element in the borders of the triangle
            side1.into_iter()
                .chain(side2.into_iter())
                .chain(side3.into_iter())
                .for_each(|p| {
                    let c = find_approximated_identifier(discretization_size, p);
                    elements.push(c);
                });

            // calcuate elements inside the triangle (not just the bordder)
            let full_elements: Vec<_> = elements
                .into_iter()
                // ordering by the x coordinate
                .chunk_by(|x| x.0)
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
                    (y_min..y_max).map(move |y| (x,y))
                })
                .filter(|c| {
                    // verify that the generated point is actually inside teh triangle
                    let point = identifier_to_zero_point(discretization_size, *c);
                    triangle.is_point_inside_footprint(point)
                })
                .collect();

            // inserting the elements in the dict
            for e in full_elements {
                let e = map
                    .entry(e)
                    .or_insert(vec![]);
                e.push(*t);
            }
        }
        
        Self {discrete_position_to_triangles: map, discretization_size}
    }

    pub fn iter_coordinates<'a> (&'a self) -> impl Iterator<Item = &'a (i32, i32)> {
        self.discrete_position_to_triangles.keys().into_iter()
    }

    /// fin the triangle that include a certain point (considering only x and y coordinate)
    pub fn find_triangle_that_includes(&self, graph: &SurfaceGraph, point: Point) -> Option<TriangleId> {
        let bucket = find_approximated_identifier(self.discretization_size, point);
        let options = self.discrete_position_to_triangles.get(&bucket)?;
        options.iter().copied().find(|id| {
            let t = graph.get_triangle(*id);
            t.is_point_inside_footprint(point)
        })
    }
}
