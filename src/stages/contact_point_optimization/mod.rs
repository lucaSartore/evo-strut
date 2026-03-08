use crate::{
    evolution::{ElitistNextGenSelector, ElitistNextGenSelectorSettings, Evolver, EvolverBehaviour, EvolverBehaviourTrait, PatienceBasedTerminationStrategy, PatienceBasedTerminationStrategySettings, Random, TournamentBasedCrossoverSelection, TournamentBasedCrossoverSelectionSettings}, models::{Settings, SurfaceGraph, TriangleId}, stages::{ContactPointsDecidedState, CriticalityGroupedState, Pipeline, PipelineBehaviourTrait, contact_point_optimization::evaluation::ContactPointEvaluatorSettings}
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

        let results: Option<Vec<_>> = (0..input.state.grouped_areas.len())
            .map(|i| {
                // todo: process elements so that the Z field is filled up
                TB::TContactPointOptimizer::optimize(&input.state, i)
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
    fn optimize(status: &CriticalityGroupedState, area_id: usize) -> Option<ContactPointsGene>;
}

pub struct SimpleContactPointOptimizer {
    
}

impl ContactPointOptimizer for SimpleContactPointOptimizer {
    fn optimize<'a>(status: &'a CriticalityGroupedState, area_id: usize) -> Option<ContactPointsGene> {
        let area = &status.grouped_areas[area_id];
        let settings = &status.settings;
        let graph = &status.graph;
        let critical = & status.critical;
        type Behaviour<'a> = EvolverBehaviour<
            ContactPointMutator,
            ContactPointCrossover,
            PatienceBasedTerminationStrategy,
            ContactPointEvaluator<'a>,
            TournamentBasedCrossoverSelection,
            ElitistNextGenSelector,
            ContactPointInitializer,
            ContactPointsGene,
            Settings,
            Settings,
            PatienceBasedTerminationStrategySettings,
            ContactPointEvaluatorSettings<'a>,
            TournamentBasedCrossoverSelectionSettings,
            ElitistNextGenSelectorSettings,
            Settings
        >;
        let evolver = Evolver::<Behaviour<'a>>::new(
            settings,
            settings,
            &PatienceBasedTerminationStrategySettings::default(),
            &ContactPointEvaluatorSettings::new(graph, settings, area, critical),
            &TournamentBasedCrossoverSelectionSettings::default(),
            &ElitistNextGenSelectorSettings::default(),
            settings,
            Random::UnSeededRandom
        );

        evolver.run()
    }
}
