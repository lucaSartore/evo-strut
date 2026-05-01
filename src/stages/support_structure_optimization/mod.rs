use anyhow::Result;
use std::{marker::PhantomData};


use crate::{evolution::{ElitistNextGenSelector, ElitistNextGenSelectorSettings, Evolver, EvolverBehaviour, PatienceBasedTerminationStrategy, PatienceBasedTerminationStrategySettings, Random, TournamentBasedCrossoverSelection, TournamentBasedCrossoverSelectionSettings}, stages::{ContactPointsDecidedState, ContactPointsGroupedState, Pipeline, PipelineBehaviourTrait, SupportStructureOptimizedState}};


mod crossover;
use crossover::{SupportStructureCrossoverSettings, SupportStructureCrossover};
mod initializer;
use initializer::{SupportStructureInitializerSettings, SupportStructureInitializer};
pub mod mutation;
use mutation::{SupportStructureMutatorSettings, SupportStructureMutator};
mod evaluation;
use evaluation::{SupportStructureEvaluatorSettings, SupportStructureEvaluator};
mod models;
pub use models::*;


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
        input: Pipeline<ContactPointsGroupedState, TB>,
    ) -> Result<Pipeline<SupportStructureOptimizedState, TB>> {
        let result = TB::TSupportStructureOptimizer::optimize(&input.state)?;

        let support_structures = result.to_full_genes(&input.state.graph);
        Ok(Pipeline::from_state(SupportStructureOptimizedState{
            settings: input.state.settings,
            graph: input.state.graph,
            support_structures,
            connection_points: input.state.connection_points
        }))
    }
}

pub trait SupportStructureOptimizer {
    fn optimize(status: &ContactPointsGroupedState) -> Result<CompressedSupportGene>;
}

pub struct SimpleSupportStructureOptimizer {
}

impl SupportStructureOptimizer for SimpleSupportStructureOptimizer {
    fn optimize<'a>(status: &'a ContactPointsGroupedState) -> Result<CompressedSupportGene> {
        let settings = &status.settings;
        let connection_points = &status.connection_points;
        let grouper = &status.grouper;
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
            CompressedSupportGene,
            SupportStructureMutatorSettings<'a>,
            SupportStructureCrossoverSettings<'a>,
            PatienceBasedTerminationStrategySettings,
            SupportStructureEvaluatorSettings<'a>,
            TournamentBasedCrossoverSelectionSettings,
            ElitistNextGenSelectorSettings,
            SupportStructureInitializerSettings<'a>
        >;
        let evolver = Evolver::<Behaviour<'a>>::new(
            &SupportStructureMutatorSettings::new(settings, graph),
            &SupportStructureCrossoverSettings::new(settings),
            &PatienceBasedTerminationStrategySettings{
                max_generations: s.num_generations,
                patience: s.patience
            },
            &SupportStructureEvaluatorSettings::new(settings, graph),
            &TournamentBasedCrossoverSelectionSettings{
                k: s.tournament_size
            },
            &ElitistNextGenSelectorSettings{
                num_novel_individual: s.generation_size - s.num_elite_individuals,
                num_elite_individual: s.num_elite_individuals
            },
            &SupportStructureInitializerSettings::new(settings, connection_points, grouper, graph),
            Random::UnSeededRandom
        );

        evolver.run()
    }
}
