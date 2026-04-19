mod models;
use log::debug;
pub use models::*;

use anyhow::Result;
use std::{marker::PhantomData};


use crate::{evolution::{ElitistNextGenSelector, ElitistNextGenSelectorSettings, Evolver, EvolverBehaviour, PatienceBasedTerminationStrategy, PatienceBasedTerminationStrategySettings, Random, TournamentBasedCrossoverSelection, TournamentBasedCrossoverSelectionSettings}, stages::{Pipeline, PipelineBehaviourTrait, SupportStructureOptimizedState, SupportStructureRefinedState}};


mod crossover;
use crossover::{SupportStructureCrossoverSettings, SupportStructureCrossover};
mod initializer;
use initializer::{SupportStructureInitializerSettings, SupportStructureInitializer};
mod mutation;
use mutation::{SupportStructureMutatorSettings, SupportStructureMutator};
pub mod evaluation;
use evaluation::{SupportStructureEvaluatorSettings, SupportStructureEvaluator};


pub struct SupportStructureRefinementStage<TB>
where
    TB: PipelineBehaviourTrait,
{
    _d: PhantomData<TB>,
}

impl<TB> SupportStructureRefinementStage<TB>
where
    TB: PipelineBehaviourTrait,
{
    pub fn execute(
        input: Pipeline<SupportStructureOptimizedState, TB>,
    ) -> Result<Pipeline<SupportStructureRefinedState, TB>> {
        let results: Result<Vec<_>> = (0..input.state.support_structures.len())
            .map(|i| {
                TB::TSupportStructureRefiner::optimize(&input.state, i)
            })
            .collect();
        Ok(Pipeline::from_state(SupportStructureRefinedState{
            settings: input.state.settings,
            graph: input.state.graph,
            connection_points: input.state.connection_points,
            support_structures: results?
        }))
    }
}

pub trait SupportStructureRefiner {
    fn optimize<'a>(status: &'a SupportStructureOptimizedState, structure_index: usize) -> Result<SupportStructureGene>;
}

pub struct SimpleSupportStructureRefiner { }

impl SupportStructureRefiner for SimpleSupportStructureRefiner {
    fn optimize<'a>(status: &'a SupportStructureOptimizedState, structure_index: usize) -> Result<SupportStructureGene> {
        debug!("starting optimization for structure {structure_index}");
        let settings = &status.settings;
        let graph = &status.graph;
        let s = &settings.support_structure_optimization_settings;
        let structure = &status.support_structures[structure_index];

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
            &SupportStructureInitializerSettings::new(settings, structure),
            Random::UnSeededRandom
        );

        evolver.run()
    }
}
