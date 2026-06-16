use anyhow::Result;
use log::debug;
use std::marker::PhantomData;

use crate::{
    evolution::{
        ElitistNextGenSelector, ElitistNextGenSelectorSettings, Evolver, EvolverBehaviour,
        PatienceBasedTerminationStrategy, PatienceBasedTerminationStrategySettings, Random,
        TournamentBasedCrossoverSelection, TournamentBasedCrossoverSelectionSettings,
    },
    stages::{
        ContactPointsDecidedState, ContactPointsGroupedState, Pipeline, PipelineBehaviourTrait,
        SupportStructureOptimizedState, floating_region_detection::FloatingRegion,
    },
};

mod crossover;
use crossover::{SupportStructureCrossover, SupportStructureCrossoverSettings};
mod initializer;
use initializer::{SupportStructureInitializer, SupportStructureInitializerSettings};
pub mod mutation;
use mutation::{SupportStructureMutator, SupportStructureMutatorSettings};
mod evaluation;
use evaluation::{SupportStructureEvaluator, SupportStructureEvaluatorSettings};
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

        let support_structures = result
            .into_iter()
            .map(|x| x.to_full_gene(&input.state.graph, &input.state.settings))
            .collect();
        Ok(Pipeline::from_state(SupportStructureOptimizedState {
            settings: input.state.settings,
            graph: input.state.graph,
            support_structures,
            connection_points: input.state.connection_points,
        }))
    }
}

pub trait SupportStructureOptimizer {
    fn optimize(status: &ContactPointsGroupedState) -> Result<Vec<SupportStructureOptimizationGene>>;
}

pub struct SimpleSupportStructureOptimizer {}

impl SimpleSupportStructureOptimizer {
    fn optimize_group<'a>(
        status: &'a ContactPointsGroupedState,
        group: &'a SupportStructureOptimizationGene,
    ) -> Result<SupportStructureOptimizationGene> {
        let floating_surfaces = FloatingRegion::filter_array(
            &status.floating_regions, 
            group.contacts.iter()
            .map(|x| x.face)
        );
        let settings = &status.settings;
        let mesh = &status.graph.mesh.original;
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
            SupportStructureOptimizationGene,
            SupportStructureMutatorSettings<'a>,
            SupportStructureCrossoverSettings<'a>,
            PatienceBasedTerminationStrategySettings,
            SupportStructureEvaluatorSettings<'a>,
            TournamentBasedCrossoverSelectionSettings,
            ElitistNextGenSelectorSettings,
            SupportStructureInitializerSettings<'a>,
        >;
        //todo: hard-coded value
        Evolver::<Behaviour<'a>>::run_n_times(
            1,
            &SupportStructureMutatorSettings::new(settings, graph),
            &SupportStructureCrossoverSettings::new(settings),
            &PatienceBasedTerminationStrategySettings {
                max_generations: s.num_generations,
                patience: s.patience,
            },
            &SupportStructureEvaluatorSettings::new(settings, graph, mesh, floating_surfaces),
            &TournamentBasedCrossoverSelectionSettings {
                k: s.tournament_size,
            },
            &ElitistNextGenSelectorSettings {
                num_novel_individual: s.generation_size - s.num_elite_individuals,
                num_elite_individual: s.num_elite_individuals,
            },
            &SupportStructureInitializerSettings::new(settings, graph, group),
            Random::UnSeededRandom,
        ).map(|x| x.0)
    }
}

impl SupportStructureOptimizer for SimpleSupportStructureOptimizer {
    fn optimize<'a>(status: &'a ContactPointsGroupedState) -> Result<Vec<SupportStructureOptimizationGene>> {
        let groups = status.grouper.to_groups(
            &status.connection_points,
            &status.graph,
            &status.settings,
            &Random::UnSeededRandom,
        );

        let mut to_return = vec![];

        for (i, g) in groups.iter().enumerate() {
            debug!("starting optimization for group {i}");
            let optimized = Self::optimize_group(status, &g);
            to_return.push(optimized?);
        }
        Ok(to_return)
    }
}
