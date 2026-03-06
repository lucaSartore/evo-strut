use std::cmp::PartialOrd;
use rayon::prelude::*;
use super::*;

pub struct Evolver<TM, TC, TT, TE, TCS, TNGS, TPI, TGene, SMut, SCross, STerm, SEval, SCrossSel, SNextSel, SInit> 
where
    TM: Mutator<TGene, SMut>,
    TC: Crossover<TGene, SCross>,
    TT: TerminationStrategy<STerm>,
    TE: Evaluator<TGene, SEval>,
    TCS: CrossoverSelector<SCrossSel>,
    TNGS: NextGenerationSelector<TGene, SNextSel>,
    TPI: PopulationInitializer<TGene, SInit>
{
    _pd: PhantomData<(TGene, SMut, SCross, STerm, SEval, SCrossSel, SNextSel, SInit)>,
    mutator: TM,
    crossover: TC,
    termination: TT,
    evaluator: TE,
    crossover_selector: TCS,
    next_gen_selector: TNGS,
    population_initializer: TPI
}

// implementation for parallel structures
impl<TM, TC, TT, TE, TCS, TNGS, TPI, TGene, SMut, SCross, STerm, SEval, SCrossSel, SNextSel, SInit>
Evolver<TM, TC, TT, TE, TCS, TNGS, TPI, TGene, SMut, SCross, STerm, SEval, SCrossSel, SNextSel, SInit>
where
    TM: Mutator<TGene, SMut> + Sync,
    TC: Crossover<TGene, SCross> + Sync,
    TT: TerminationStrategy<STerm> + Sync,
    TE: Evaluator<TGene, SEval> + Sync,
    TCS: CrossoverSelector<SCrossSel> + Sync,
    TNGS: NextGenerationSelector<TGene, SNextSel> + Sync,
    TPI: PopulationInitializer<TGene, SInit> + Sync,
    TGene: Sync + Send, SMut: Sync, SCross: Sync, STerm: Sync, SEval: Sync, SCrossSel: Sync, SNextSel: Sync, SInit: Sync
{
    pub fn new(
        mutation_settings: &SMut,
        crossover_settings: &SCross,
        termination_settings: &STerm,
        evaluation_settings: &SEval,
        crossover_selection_settings: &SCrossSel,
        next_generation_settings: &SNextSel,
        population_initializer_settings: &SInit,
        mut rand: Random
    ) -> Self {
        Self {
            _pd: PhantomData::default(),
            mutator: TM::new(mutation_settings, rand.seeded_copy()),
            crossover: TC::new(crossover_settings, rand.seeded_copy()),
            termination: TT::new(termination_settings),
            evaluator: TE::new(evaluation_settings),
            crossover_selector: TCS::new(crossover_selection_settings, rand.seeded_copy()),
            next_gen_selector: TNGS::new(next_generation_settings, rand.seeded_copy()),
            population_initializer: TPI::new(population_initializer_settings, rand.seeded_copy())
        }
    }

    pub fn run(&self) -> Option<TGene> {

        let best = |x: &[Cost]| {
            x
                .iter()
                .copied()
                .min()
                .unwrap_or(Cost::MAX)
        };

        let initial_count = self.population_initializer.get_initial_individuals();
        let mut current_gen: Vec<_> = (0..initial_count)
            .map(|_| self.population_initializer.get_random_individual())
            .collect();
        
        let mut current_gen_costs = current_gen
            .par_iter()
            .map(|y| self.evaluator.evaluate(y))
            .collect::<Vec<Cost>>();

        while !self.termination.should_terminate(best(&current_gen_costs)) {

            let n = self.next_gen_selector.num_offspring_to_generate();
            let (next_gen, next_gen_costs): (Vec<_>, Vec<_>) = self
                .crossover_selector
                .select_for_crossover(&current_gen_costs, n)?
                .par_iter()
                .map(|(ai, bi)| {
                    let a = &current_gen[*ai];
                    let b = &current_gen[*bi];
                    let new_individual = self.crossover.crossover(a, b);
                    let new_individual = self.mutator.mutate(new_individual);
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

            (current_gen, current_gen_costs) = self.next_gen_selector.next_generation(last_gen, last_gen_costs, next_gen, next_gen_costs);
        }

        current_gen
            .into_iter()
            .zip(current_gen_costs.iter())
            .min_by(|a,b| {
                a.1.partial_cmp(b.1)
                   .unwrap_or(std::cmp::Ordering::Less)
            })
            .map_or(
                self.population_initializer.get_random_individual(),
                |x| x.0
            )
            .into()
    }
}
