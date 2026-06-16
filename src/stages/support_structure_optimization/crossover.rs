use super::SupportStructureOptimizationGene;
use crate::{
    evolution::{Crossover, Random},
    models::{Plane, Settings},
    support::neural_network::NetworkCrossoverSettings,
};

pub struct SupportStructureCrossoverSettings<'a> {
    settings: &'a Settings,
}

impl<'a> SupportStructureCrossoverSettings<'a> {
    pub fn new(settings: &'a Settings) -> Self {
        Self { settings }
    }
}

pub struct SupportStructureCrossover<'a> {
    settings: &'a Settings,
    rand: Random,
}

impl<'a> Crossover<SupportStructureOptimizationGene, SupportStructureCrossoverSettings<'a>>
    for SupportStructureCrossover<'a>
{
    fn new(settings: &SupportStructureCrossoverSettings<'a>, rand: Random) -> Self {
        Self {
            settings: settings.settings,
            rand,
        }
    }

    fn crossover(
        &self,
        a: &SupportStructureOptimizationGene,
        b: &SupportStructureOptimizationGene,
    ) -> SupportStructureOptimizationGene {
        let rand = &self.rand;
        let p1 = a.random_point(rand);
        let p2 = a.random_point(rand);
        let p3 = b.random_point(rand);

        let plane = Plane::from_points_and_max_distance(p1, p2, p3);

        let supports_from_a = a
            .supports
            .iter()
            .filter(|x| plane.classify_point(x.position));

        let supports_from_b = b
            .supports
            .iter()
            .filter(|x| !plane.classify_point(x.position));

        let valid_crossovers = vec![
            NetworkCrossoverSettings::uniform(),
            NetworkCrossoverSettings::single_point(),
            NetworkCrossoverSettings::arithmetic(0.5)
                .expect("invalid default contact-point grouping crossover settings"),
        ];

        let crossover = rand
            .choose(&valid_crossovers)
            .expect("at least one crossover should be found");

        SupportStructureOptimizationGene {
            contacts: a.contacts.clone(),
            supports: supports_from_a.chain(supports_from_b).copied().collect(),
            contact_radius: a
                .contact_radius
                .crossover(&b.contact_radius, crossover, rand)
                .expect("network crossover failed"),
            convex_hull: a.convex_hull.clone(),
        }
    }
}
