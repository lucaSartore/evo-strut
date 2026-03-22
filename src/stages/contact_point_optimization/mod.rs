use crate::{
    evolution::{ElitistNextGenSelector, ElitistNextGenSelectorSettings, Evolver, EvolverBehaviour, PatienceBasedTerminationStrategy, PatienceBasedTerminationStrategySettings, Random, TournamentBasedCrossoverSelection, TournamentBasedCrossoverSelectionSettings}, models::Settings, stages::{ContactPointsDecidedState, CriticalityGroupedState, Pipeline, PipelineBehaviourTrait, contact_point_optimization::{evaluation::ContactPointEvaluatorSettings, initializer::ContactPointsInitializerSettings}}
};
use anyhow::{Result, anyhow};
use log::debug;
use std::{hash::Hash, marker::PhantomData};

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
    ) -> Result<Pipeline<ContactPointsDecidedState, TB>> {

        // let merged = TB::TContactPointOptimizer::optimize(&input.state, 0)?;
        println!("num grouped areas: {}", input.state.grouped_areas.len());
        let results: Result<Vec<_>> = (0..input.state.grouped_areas.len())
            .map(|i| {
                // todo: process elements so that the Z field is filled up
                TB::TContactPointOptimizer::optimize(&input.state, i)
            })
            .collect();
        let merged = ContactPointsGene::merge_many(results?)
            .ok_or(anyhow!("merging of multiple genes failed"))?;

        Ok(Pipeline::from_state(ContactPointsDecidedState{
            settings: input.state.settings,
            graph: input.state.graph,
            connection_points: merged
        }))
    }
}

pub trait ContactPointOptimizer {
    fn optimize(status: &CriticalityGroupedState, area_id: usize) -> Result<ContactPointsGene>;
}

pub struct SimpleContactPointOptimizer {
    
}

impl ContactPointOptimizer for SimpleContactPointOptimizer {
    fn optimize<'a>(status: &'a CriticalityGroupedState, area_id: usize) -> Result<ContactPointsGene> {
        debug!("starting optimization for area {area_id}");
        let area = &status.grouped_areas[area_id];
        let area_hash = &status.grouped_areas_hashes[area_id];
        let settings = &status.settings;
        let graph = &status.graph;
        let critical = & status.critical;
        type Behaviour<'a> = EvolverBehaviour<
            ContactPointMutator<'a>,
            ContactPointCrossover,
            PatienceBasedTerminationStrategy,
            ContactPointEvaluator<'a>,
            TournamentBasedCrossoverSelection,
            ElitistNextGenSelector,
            ContactPointInitializer<'a>,
            ContactPointsGene,
            ContactPointsInitializerSettings<'a>,
            Settings,
            PatienceBasedTerminationStrategySettings,
            ContactPointEvaluatorSettings<'a>,
            TournamentBasedCrossoverSelectionSettings,
            ElitistNextGenSelectorSettings,
            ContactPointsInitializerSettings<'a>
        >;
        let evolver = Evolver::<Behaviour<'a>>::new(
            &ContactPointsInitializerSettings::new(settings, graph, area, area_hash),
            settings,
            &PatienceBasedTerminationStrategySettings::default(),
            &ContactPointEvaluatorSettings::new(graph, settings, area, critical, area_id),
            &TournamentBasedCrossoverSelectionSettings::default(),
            &ElitistNextGenSelectorSettings::default(),
            &ContactPointsInitializerSettings::new(settings, graph, area, area_hash),
            Random::UnSeededRandom
        );

        evolver.run()
    }
}
