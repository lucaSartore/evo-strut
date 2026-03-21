use super::*;
use log::{debug, info};
use rayon::prelude::*;
use anyhow::{Result, anyhow};

pub trait EvolverBehaviourTrait {
    // behaviour of the various components of teh GA
    type TMutator: Mutator<Self::TGene, Self::SMut>;
    type TCrossover: Crossover<Self::TGene, Self::SCross>;
    type TTermination: TerminationStrategy<Self::STerm>;
    type TEvaluator: Evaluator<Self::TGene, Self::SEval>;
    type TCrossoverSelector: CrossoverSelector<Self::SCrossSel>;
    type TNextGenSelector: NextGenerationSelector<Self::TGene, Self::SNextSel>;
    type TPopulationInitializer: PopulationInitializer<Self::TGene, Self::SInit>;
    // type of the gene
    type TGene: Clone;
    // settings for the various components
    type SMut;
    type SCross;
    type STerm;
    type SEval;
    type SCrossSel;
    type SNextSel;
    type SInit;
}

pub struct Evolver<TBehaviour>
where
    TBehaviour: EvolverBehaviourTrait,
{
    mutator: TBehaviour::TMutator,
    crossover: TBehaviour::TCrossover,
    termination: TBehaviour::TTermination,
    evaluator: TBehaviour::TEvaluator,
    crossover_selector: TBehaviour::TCrossoverSelector,
    next_gen_selector: TBehaviour::TNextGenSelector,
    population_initializer: TBehaviour::TPopulationInitializer,
}

// implementation for parallel structures
impl<TBehaviour> Evolver<TBehaviour>
where
    TBehaviour: EvolverBehaviourTrait,
    TBehaviour::TGene: Send + Sync,
    TBehaviour::TMutator: Sync,
    TBehaviour::TCrossover: Sync,
    TBehaviour::TTermination: Sync,
    TBehaviour::TEvaluator: Sync,
    TBehaviour::TCrossoverSelector: Sync,
    TBehaviour::TNextGenSelector: Sync,
    TBehaviour::TPopulationInitializer: Sync,
{
    pub fn new(
        mutation_settings: &TBehaviour::SMut,
        crossover_settings: &TBehaviour::SCross,
        termination_settings: &TBehaviour::STerm,
        evaluation_settings: &TBehaviour::SEval,
        crossover_selection_settings: &TBehaviour::SCrossSel,
        next_generation_settings: &TBehaviour::SNextSel,
        population_initializer_settings: &TBehaviour::SInit,
        rand: Random,
    ) -> Self {
        Self {
            mutator: TBehaviour::TMutator::new(mutation_settings, rand.seeded_copy()),
            crossover: TBehaviour::TCrossover::new(crossover_settings, rand.seeded_copy()),
            termination: TBehaviour::TTermination::new(termination_settings),
            evaluator: TBehaviour::TEvaluator::new(evaluation_settings),
            crossover_selector: TBehaviour::TCrossoverSelector::new(
                crossover_selection_settings,
                rand.seeded_copy(),
            ),
            next_gen_selector: TBehaviour::TNextGenSelector::new(
                next_generation_settings,
                rand.seeded_copy(),
            ),
            population_initializer: TBehaviour::TPopulationInitializer::new(
                population_initializer_settings,
                rand.seeded_copy(),
            ),
        }
    }

    pub fn run(&self) -> Result<TBehaviour::TGene> {
        let best = |x: &[Cost]| x.iter().copied().min().unwrap_or(Cost::MAX);

        let initial_count = self.population_initializer.get_initial_individuals();
        let mut current_gen: Vec<_> = (0..initial_count)
            .map(|_| self.population_initializer.get_random_individual())
            .collect();

        let mut current_gen_costs = current_gen
            // .par_iter()
            .iter()
            .map(|y| self.evaluator.evaluate(y))
            .collect::<Vec<Cost>>();

        let mut counter = 0;
        while !self.termination.should_terminate(best(&current_gen_costs)) {
            let n = self.next_gen_selector.num_offspring_to_generate();
            let (next_gen, next_gen_costs): (Vec<_>, Vec<_>) = self
                .crossover_selector
                .select_for_crossover(&current_gen_costs, n)
                .ok_or(anyhow!("Crossover selection failed to return any element"))?
                .par_iter()
                // .iter()
                .map(|(ai, bi)| {
                    let a = &current_gen[*ai];
                    let b = &current_gen[*bi];
                    let mut new_individual = self.crossover.crossover(a, b);
                    self.mutator.mutate(&mut new_individual);
                    let cost = self.evaluator.evaluate(&new_individual);
                    (new_individual, cost)
                })
                .collect::<Vec<_>>()
                .into_iter()
                .unzip();

            let mut last_gen = vec![];
            let mut last_gen_costs = vec![];

            std::mem::swap(&mut last_gen, &mut current_gen);
            std::mem::swap(&mut last_gen_costs, &mut current_gen_costs);

            (current_gen, current_gen_costs) = self.next_gen_selector.next_generation(
                last_gen,
                last_gen_costs,
                next_gen,
                next_gen_costs,
            );

            let (best, best_cost) = current_gen
                .iter()
                .zip(current_gen_costs.iter())
                .min_by_key(|(_,c)| **c)
                .ok_or(anyhow!("Unable to find best individual for evaluation"))?;

            counter += 1;
            info!("Generation {counter} best cost was {best_cost}");

            if counter % 10 == 0 {
                self.evaluator.visualize(best)?;
            }
        }

        let x = current_gen
            .into_iter()
            .zip(current_gen_costs.iter())
            .min_by_key(|(_,c)| **c)
            .ok_or(anyhow!("Unable to find best individual, the list was empty"))?;
        Ok(x.0)
    }
}

pub struct EvolverBehaviour<
    TMutator, TCrossover, TTermination, TEvaluator, TCrossoverSelector, 
    TNextGenSelector, TPopulationInitializer, TGene, SMut, SCross, 
    STerm, SEval, SCrossSel, SNextSel, SInit
> {
    _marker: PhantomData<(
        TMutator, TCrossover, TTermination, TEvaluator, TCrossoverSelector, 
        TNextGenSelector, TPopulationInitializer, TGene, SMut, SCross, 
        STerm, SEval, SCrossSel, SNextSel, SInit
    )>,
}

impl<
    TMutator, TCrossover, TTermination, TEvaluator, TCrossoverSelector, 
    TNextGenSelector, TPopulationInitializer, TGene, SMut, SCross, 
    STerm, SEval, SCrossSel, SNextSel, SInit
> EvolverBehaviourTrait for EvolverBehaviour<
    TMutator, TCrossover, TTermination, TEvaluator, TCrossoverSelector, 
    TNextGenSelector, TPopulationInitializer, TGene, SMut, SCross, 
    STerm, SEval, SCrossSel, SNextSel, SInit
> 
where
    TMutator: Mutator<TGene, SMut>,
    TCrossover: Crossover<TGene, SCross>,
    TTermination: TerminationStrategy<STerm>,
    TEvaluator: Evaluator<TGene, SEval>,
    TCrossoverSelector: CrossoverSelector<SCrossSel>,
    TNextGenSelector: NextGenerationSelector<TGene, SNextSel>,
    TPopulationInitializer: PopulationInitializer<TGene, SInit>,
    TGene: Clone + 'static
{
    type TMutator = TMutator;
    type TCrossover = TCrossover;
    type TTermination = TTermination;
    type TEvaluator = TEvaluator;
    type TCrossoverSelector = TCrossoverSelector;
    type TNextGenSelector = TNextGenSelector;
    type TPopulationInitializer = TPopulationInitializer;
    
    type TGene = TGene;
    
    type SMut = SMut;
    type SCross = SCross;
    type STerm = STerm;
    type SEval = SEval;
    type SCrossSel = SCrossSel;
    type SNextSel = SNextSel;
    type SInit = SInit;
}
