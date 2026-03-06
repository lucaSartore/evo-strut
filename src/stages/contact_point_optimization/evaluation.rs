use crate::{evolution::{Evaluator, Cost}, models::{Settings, SurfaceGraph, TriangleId}, stages::contact_point_optimization::models::ContactPointsGene};

pub struct  ContactPointEvaluator {
}

impl Evaluator<ContactPointsGene, Settings> for ContactPointEvaluator {
    fn new(settings: &Settings) -> Self {
        todo!()
    }

    fn evaluate(&self, gene: &ContactPointsGene) -> Cost {
        todo!()
    }
}
