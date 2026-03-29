mod models;
use log::debug;
pub use models::*;

use anyhow::Result;
use std::marker::PhantomData;


use crate::{evolution::{ElitistNextGenSelector, ElitistNextGenSelectorSettings, Evolver, EvolverBehaviour, PatienceBasedTerminationStrategy, PatienceBasedTerminationStrategySettings, Random, TournamentBasedCrossoverSelection, TournamentBasedCrossoverSelectionSettings}, stages::{ContactPointsDecidedState, Pipeline, PipelineBehaviourTrait, SupportStructureOptimizedState}};


mod crossover;
use crossover::{SupportStructureCrossoverSettings, SupportStructureCrossover};
mod initializer;
use initializer::{SupportStructureInitializerSettings, SupportStructureInitializer};
mod mutation;
use mutation::{SupportStructureMutatorSettings, SupportStructureMutator};
mod evaluation;
use evaluation::{SupportStructureEvaluatorSettings, SupportStructureEvaluator};


pub struct SupportStructureOptimizationStage<TB>
where
    TB: PipelineBehaviourTrait,
{
    _d: PhantomData<TB>,
}

impl<TB> SupportStructureOptimizationStage<TB>
where
    TB: PipelineBehaviourTrait,
{
    pub fn execute(
        input: Pipeline<ContactPointsDecidedState, TB>,
    ) -> Result<Pipeline<SupportStructureOptimizedState, TB>> {
        let result = TB::TSupportStructureOptimizer::optimize(&input.state)?;
        Ok(Pipeline::from_state(SupportStructureOptimizedState{
            settings: input.state.settings,
            graph: input.state.graph,
            connection_points: input.state.connection_points,
            support_structure: result
        }))
    }
}

pub trait SupportStructureOptimizer {
    fn optimize(status: &ContactPointsDecidedState) -> Result<SupportStructureGene>;
}

pub struct SimpleSupportStructureOptimizer {
}

impl SupportStructureOptimizer for SimpleSupportStructureOptimizer {
    fn optimize<'a>(status: &'a ContactPointsDecidedState) -> Result<SupportStructureGene> {
        let settings = &status.settings;
        let graph = &status.graph;
        let s = &settings.support_structure_optimization_settings;

        type Behaviour<'a> = EvolverBehaviour<
            SupportStructureMutator<'a>,
            SupportStructureCrossover<'a>,
            PatienceBasedTerminationStrategy,
            SupportStructureEvaluator<'a>,
            TournamentBasedCrossoverSelection,
            ElitistNextGenSelector,
            SupportStructureInitializer<'a>,
            SupportStructureGene,
            SupportStructureMutatorSettings<'a>,
            SupportStructureCrossoverSettings<'a>,
            PatienceBasedTerminationStrategySettings,
            SupportStructureEvaluatorSettings<'a>,
            TournamentBasedCrossoverSelectionSettings,
            ElitistNextGenSelectorSettings,
            SupportStructureInitializerSettings<'a>
        >;
        let evolver = Evolver::<Behaviour<'a>>::new(
            &SupportStructureMutatorSettings::new(settings),
            &SupportStructureCrossoverSettings::new(settings),
            &PatienceBasedTerminationStrategySettings{
                max_generations: s.num_generations,
                patience: s.patience
            },
            &SupportStructureEvaluatorSettings::new(settings),
            &TournamentBasedCrossoverSelectionSettings{
                k: s.tournament_size
            },
            &ElitistNextGenSelectorSettings{
                num_novel_individual: s.generation_size - s.num_elite_individuals,
                num_elite_individual: s.num_elite_individuals
            },
            &SupportStructureInitializerSettings::new(settings),
            Random::UnSeededRandom
        );

        evolver.run()
    }
}
