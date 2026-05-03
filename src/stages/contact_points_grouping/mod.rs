use anyhow::Result;
use std::marker::PhantomData;

use crate::{
    evolution::{
        ElitistNextGenSelector, ElitistNextGenSelectorSettings, Evolver, EvolverBehaviour,
        PatienceBasedTerminationStrategy, PatienceBasedTerminationStrategySettings, Random,
        TournamentBasedCrossoverSelection, TournamentBasedCrossoverSelectionSettings,
    },
    stages::{
        support_structure_refinement::evaluation::SupportStructureEvaluatorSettings,
        ContactPointsDecidedState, ContactPointsGroupedState, Pipeline, PipelineBehaviourTrait,
        SupportStructureOptimizedState,
    },
};

mod crossover;
use crossover::{ContactPointGroupingCrossover, ContactPointsGroupingSettings};
mod initializer;
use initializer::{ContactPointGroupingInitializer, ContactPointGroupingInitializerSettings};
mod mutation;
use mutation::{ContactPointGroupingMutator, ContactPointGroupingMutatorSettings};
mod evaluation;
use evaluation::{ContactPointGroupingEvaluator, ContactPointGroupingEvaluatorSettings};
mod models;
pub use models::*;

pub struct ContactPointsGroupingStage<TB>
where
    TB: PipelineBehaviourTrait,
{
    _d: PhantomData<TB>,
}

impl<TB> ContactPointsGroupingStage<TB>
where
    TB: PipelineBehaviourTrait,
{
    pub fn execute(
        input: Pipeline<ContactPointsDecidedState, TB>,
    ) -> Result<Pipeline<ContactPointsGroupedState, TB>> {
        let result = SimpleContactPointsGrouper::optimize(&input.state)?;

        Ok(Pipeline::from_state(ContactPointsGroupedState {
            settings: input.state.settings,
            graph: input.state.graph,
            connection_points: input.state.connection_points,
            grouper: result,
        }))
    }
}

pub trait ContactPointsGrouper {
    fn optimize(status: &ContactPointsDecidedState) -> Result<ContactPointGroupingGene>;
}

pub struct SimpleContactPointsGrouper {}

impl ContactPointsGrouper for SimpleContactPointsGrouper {
    fn optimize<'a>(status: &'a ContactPointsDecidedState) -> Result<ContactPointGroupingGene> {
        let settings = &status.settings;
        let connection_points = &status.connection_points;
        let graph = &status.graph;
        let s = &settings.contact_points_grouping_settings;

        type Behaviour<'a> = EvolverBehaviour<
            ContactPointGroupingMutator<'a>,
            ContactPointGroupingCrossover<'a>,
            PatienceBasedTerminationStrategy,
            ContactPointGroupingEvaluator<'a>,
            TournamentBasedCrossoverSelection,
            ElitistNextGenSelector,
            ContactPointGroupingInitializer<'a>,
            ContactPointGroupingGene,
            ContactPointGroupingMutatorSettings<'a>,
            ContactPointsGroupingSettings<'a>,
            PatienceBasedTerminationStrategySettings,
            ContactPointGroupingEvaluatorSettings<'a>,
            TournamentBasedCrossoverSelectionSettings,
            ElitistNextGenSelectorSettings,
            ContactPointGroupingInitializerSettings<'a>,
        >;
        let grouping_evaluator_settings = ContactPointGroupingEvaluatorSettings::new(
            settings,
            graph,
            Random::UnSeededRandom,
            connection_points,
        );

        let evolver = Evolver::<Behaviour<'a>>::new(
            &ContactPointGroupingMutatorSettings::new(settings, graph),
            &ContactPointsGroupingSettings::new(settings),
            &PatienceBasedTerminationStrategySettings {
                max_generations: s.num_generations,
                patience: s.patience,
            },
            &grouping_evaluator_settings,
            &TournamentBasedCrossoverSelectionSettings {
                k: s.tournament_size,
            },
            &ElitistNextGenSelectorSettings {
                num_novel_individual: s.generation_size - s.num_elite_individuals,
                num_elite_individual: s.num_elite_individuals,
            },
            &ContactPointGroupingInitializerSettings::new(settings, connection_points, graph),
            Random::UnSeededRandom,
        );

        evolver.run()
    }
}
