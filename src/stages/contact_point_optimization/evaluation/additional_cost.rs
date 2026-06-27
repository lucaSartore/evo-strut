use kdtree::distance::squared_euclidean;
use kdtree::KdTree;
use log::warn;
use rerun::external::crossbeam::epoch::default_collector;
use std::{cell::RefCell, collections::HashMap};

use crate::{
    evolution::Cost,
    models::{Point, PointI},
};

/// ad additional costs to a node, depending on his position.
/// the cost added is proportional to the distance to the nearest support
/// to encourage the optimizer to spread out points evenly
pub struct AdditionalCosts {
    /// cache for the results
    cache: RefCell<HashMap<PointI, Cost>>,
    /// divisor (used to bundle up multiple close-points in one single
    /// cache entry)
    divisor: f32,
    /// multiplier used to convert distance into cost
    /// unit of measure: cost/mm
    cost_multiplier: f32,
    /// ceiling for the maximum distance
    max_distance: f32,
    /// points that are considered when calculating
    /// the min distance
    num_parallel_points: usize,
    positions: KdTree<f32, (), [f32; 3]>,
}

impl AdditionalCosts {
    pub fn new(
        points: impl Iterator<Item = Point>,
        cost_multiplier: f32,
        divisor: f32,
        max_distance: f32,
        num_parallel_points: usize,
    ) -> Self {
        let mut positions: KdTree<f32, (), [f32; 3]> = KdTree::new(3);

        for point in points {
            positions
                .add(point.into(), ())
                .expect("Add point to kd-tree failed unexpectedly");
        }

        Self {
            cache: Default::default(),
            divisor,
            cost_multiplier,
            max_distance,
            positions,
            num_parallel_points,
        }
    }

    pub fn evaluate(&self, point: Point) -> Cost {
        let point_i = PointI::new(point, self.divisor);
        if let Some(x) = self.cache.borrow().get(&point_i) {
            return *x;
        }
        // re-calculate float point to avoid having different output
        // depending on which point call "evaluate" first
        let cost = self._evaluate(point_i.to_float(self.divisor));
        self.cache.borrow_mut().insert(point_i, cost);
        cost
    }
    fn _evaluate(&self, point: Point) -> Cost {
        let point_coordinates: [f32; 3] = point.into();
        let closest = self
            .positions
            .nearest(&point_coordinates, 1, &squared_euclidean)
            .expect("nearest point search has failed unexpectedly");
        let distances: Vec<_> = closest
            .iter()
            .take(self.num_parallel_points)
            .map(|x| x.0.sqrt().min(self.max_distance))
            .collect();

        let distance = self._distance_parallel(&distances);

        Cost::new(distance * self.cost_multiplier)
    }

    fn _distance_parallel(&self, distance: &[f32]) -> f32 {
        if distance.is_empty() {
            return self.max_distance;
        }

        if distance
            .iter()
            .any(|x| *x < f32::EPSILON * self.num_parallel_points as f32)
        {
            return 0.0;
        }

        let result = distance
            .iter()
            .map(|x| (*x as f64).powi(-1))
            .sum::<f64>()
            .powi(-1);
        result as f32
    }
}
