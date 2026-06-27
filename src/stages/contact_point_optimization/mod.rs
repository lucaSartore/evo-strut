use crate::{
    evolution::{
        ElitistNextGenSelector, ElitistNextGenSelectorSettings, Evolver, EvolverBehaviour,
        PatienceBasedTerminationStrategy, PatienceBasedTerminationStrategySettings, Random,
        TournamentBasedCrossoverSelection, TournamentBasedCrossoverSelectionSettings,
    },
    stages::{
        contact_point_optimization::{
            corssover::ContactPointCrossoverSettings, evaluation::ContactPointEvaluatorSettings,
            initializer::ContactPointsInitializerSettings, mutation::ContactPointsMutatorSettings,
        },
        save_optimization_artifact, ContactPointsDecidedState, CriticalityGroupedState, Pipeline,
        PipelineBehaviourTrait,
    },
};
use anyhow::{anyhow, Result};
use log::debug;
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
    ) -> Result<Pipeline<ContactPointsDecidedState, TB>> {
        // let merged = TB::TContactPointOptimizer::optimize(&input.state, 0)?;
        println!("num grouped areas: {}", input.state.grouped_areas.len());
        let results: Result<Vec<_>> = (0..input.state.grouped_areas.len())
            .map(|i| TB::TContactPointOptimizer::optimize(&input.state, i))
            .collect();
        let merged = ContactPointsGene::merge_many(results?)
            .ok_or(anyhow!("merging of multiple genes failed"))?;

        Ok(Pipeline::from_state(ContactPointsDecidedState {
            settings: input.state.settings,
            graph: input.state.graph,
            connection_points: merged,
            critical: input.state.critical,
            floating_regions: input.state.floating_regions,
        }))
    }
}

pub trait ContactPointOptimizer {
    fn optimize(status: &CriticalityGroupedState, area_id: usize) -> Result<ContactPointsGene>;
}

pub struct SimpleContactPointOptimizer {}

impl ContactPointOptimizer for SimpleContactPointOptimizer {
    fn optimize<'a>(
        status: &'a CriticalityGroupedState,
        area_id: usize,
    ) -> Result<ContactPointsGene> {
        debug!("starting optimization for area {area_id}");
        let area = &status.grouped_areas[area_id];
        let area_hash = &status.grouped_areas_hashes[area_id];
        let settings = &status.settings;
        let graph = &status.graph;
        let s = &settings.contact_points_optimization_settings;
        type Behaviour<'a> = EvolverBehaviour<
            ContactPointMutator<'a>,
            ContactPointCrossover<'a>,
            PatienceBasedTerminationStrategy,
            ContactPointEvaluator<'a>,
            TournamentBasedCrossoverSelection,
            ElitistNextGenSelector,
            ContactPointInitializer<'a>,
            ContactPointsGene,
            ContactPointsMutatorSettings<'a>,
            ContactPointCrossoverSettings<'a>,
            PatienceBasedTerminationStrategySettings,
            ContactPointEvaluatorSettings<'a>,
            TournamentBasedCrossoverSelectionSettings,
            ElitistNextGenSelectorSettings,
            ContactPointsInitializerSettings<'a>,
        >;
        let evolver = Evolver::<Behaviour<'a>>::new(
            &ContactPointsMutatorSettings::new(settings, graph, area, area_hash),
            &ContactPointCrossoverSettings::new(area, graph),
            &PatienceBasedTerminationStrategySettings {
                max_generations: s.num_generations,
                patience: s.patience,
            },
            &ContactPointEvaluatorSettings::new(graph, settings, area, area_id),
            &TournamentBasedCrossoverSelectionSettings {
                k: s.tournament_size,
            },
            &ElitistNextGenSelectorSettings {
                num_novel_individual: s.generation_size - s.num_elite_individuals,
                num_elite_individual: s.num_elite_individuals,
            },
            &ContactPointsInitializerSettings::new(settings, graph, area),
            Random::UnSeededRandom,
        );

        let (result, cost_log) = evolver.run_once()?;
        save_optimization_artifact(
            settings,
            format!("contact_points_optimization_area_{area_id}.json"),
            &result,
            &cost_log,
        )?;
        Ok(result)
    }
}
