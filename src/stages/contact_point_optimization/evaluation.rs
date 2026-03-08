use std::collections::HashMap;
use crate::{evolution::{Cost, Evaluator}, models::{Point, Settings, SurfaceGraph, TriangleId}, stages::contact_point_optimization::models::ContactPointsGene};
use itertools::Itertools;
use log::info;

pub struct ContactPointEvaluator<'a> {
    graph: &'a SurfaceGraph,
    settings: Settings,
    critical: &'a [TriangleId],
    discrete_position_to_triangles: HashMap<(i32, i32), Vec<TriangleId>>,
    discretization_size: f32
}

impl ContactPointEvaluator<'_> {
    fn calculate_discretization_size(&mut self) {
        self.discretization_size = 1.0 * self.critical
            .iter()
            .map(|x| {
                let t = self.graph.get_triangle(*x);
                t.area().sqrt()
            })
            .sum::<f32>() / (self.critical.len()) as f32;
        info!(
            "ContactPointEvaluator: discretization size selected: {}",
            self.discretization_size
        );
    }

    pub fn find_approximated_identifier(&self, point: Point) -> (i32, i32) {
        let x = (point.x / self.discretization_size).round() as i32;
        let y = (point.y / self.discretization_size).round() as i32;
        (x,y)
    }

    fn fill_discretization_position(&mut self) {
        // for each triangle
        for t in self.critical {
            let triangle = self.graph.get_triangle(*t);
            let mut elements = vec![];
            let [v1, v2, v3] = triangle.vertexes();

            // we sample points along the vertexes (with a size that make shure
            // se place a point into all "bukets"
            let side1 = Point::interpolate(v1, v2, self.discretization_size * 0.4);
            let side2 = Point::interpolate(v2, v3, self.discretization_size * 0.4);
            let side3 = Point::interpolate(v3, v1, self.discretization_size * 0.4);

            // generating an identifier for each element in the borders of the triangle
            side1.into_iter()
                .chain(side2.into_iter())
                .chain(side3.into_iter())
                .for_each(|p| {
                    let c = self.find_approximated_identifier(p);
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
                .collect();

            // inserting the elements in the dict
            for e in full_elements {
                let e = self
                    .discrete_position_to_triangles
                    .entry(e)
                    .or_insert(vec![]);
                e.push(*t);
            }
        }
    }

    fn log_infos(&self) {
        let num_triangles = self.critical.len();
        let num_buckets = self.discrete_position_to_triangles.len();
        let num_bucketed_triangles = self
            .discrete_position_to_triangles
            .values()
            .fold(0, |acc, v| acc + v.len());
        info!("ContactPointEvaluator creation:");
        info!("\t {} total triangle in the sruface", num_triangles);
        info!("\t {} total buckets", num_buckets);
        info!("\t {} average triangles in each bucket", num_bucketed_triangles as f32 / num_buckets as f32);
    }
}
impl<'a> Evaluator<ContactPointsGene, (&'a SurfaceGraph, &'a Settings, &'a [TriangleId])> for ContactPointEvaluator<'a> {
    fn new(settings: &(&'a SurfaceGraph, &'a Settings, &'a [TriangleId])) -> Self {
        let mut s = Self {
            graph: settings.0,
            settings: settings.1.clone(),
            critical: settings.2,
            discrete_position_to_triangles: HashMap::default(),
            discretization_size: 0.0
        };
        s.calculate_discretization_size();
        s.fill_discretization_position();
        s.log_infos();
        s
    }

    fn evaluate(&self, gene: &ContactPointsGene) -> Cost {
        todo!()
    }
}
