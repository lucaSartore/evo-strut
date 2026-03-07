use crate::{
    evolution::EvolverBehaviourTrait, models::{Settings, SurfaceGraph, TriangleId}, stages::{ContactPointsDecidedState, CriticalityGroupedState, Pipeline, PipelineBehaviourTrait}
};
use std::marker::PhantomData;

mod corssover;
pub use corssover::ContactPointCrossover;
mod evaluation;
pub use evaluation::ContactPointEvaluator;
mod initializer;
pub use initializer::ContactPointInitializer;
mod models;
pub use models::*;
mod mutation;
pub use mutation::ContactPointMutator;

pub struct ContactPointOptimizationStage<TB>
where
    TB: PipelineBehaviourTrait,
{
    _d: PhantomData<TB>,
}

impl<TB> ContactPointOptimizationStage<TB>
where
    TB: PipelineBehaviourTrait,
{
    pub fn execute(
        input: Pipeline<CriticalityGroupedState, TB>,
    ) -> Option<Pipeline<ContactPointsDecidedState, TB>> {

        let results: Option<Vec<_>> = input
            .state
            .critical
            .iter()
            .map(|x| {
                // todo: process elements so that the Z field is filled up
                TB::TContactPointOptimizer::optimize(&input.state.graph, &input.state.settings, x)
            })
            .collect();

        let merged = ContactPointsGene::merge_many(results?)?;
        
        Pipeline::from_state(ContactPointsDecidedState{
            settings: input.state.settings,
            graph: input.state.graph,
            connection_points: merged
        }).into()
    }
}

pub trait ContactPointOptimizer {
    fn optimize(graph: &SurfaceGraph, settings: &Settings, critical: &[TriangleId]) -> Option<ContactPointsGene>;
}

pub struct SimpleContactPointOptimizer {
    
}

impl ContactPointOptimizer for SimpleContactPointOptimizer {
    fn optimize(graph: &SurfaceGraph, settings: &Settings, critical: &[TriangleId]) -> Option<ContactPointsGene> {
        todo!()
    }
}
