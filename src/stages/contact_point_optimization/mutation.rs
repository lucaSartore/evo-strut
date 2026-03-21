use hashbrown::HashSet;

use crate::{evolution::{Mutator, Random}, models::{FaceId, Settings, SurfaceGraph}, stages::contact_point_optimization::initializer::ContactPointsInitializerSettings, support::remove_random::RemoveRandom};
use super::models::ContactPointsGene;


pub struct ContactPointMutator<'a> {
    settings: &'a Settings,
    graph: &'a SurfaceGraph,
    options: &'a [FaceId],
    options_hash: &'a HashSet<FaceId>,
    rand: Random
}

impl<'a> ContactPointMutator<'a> {
    fn add_support_mutation(&self, gene: &mut ContactPointsGene) {
        let to_add = self.rand.choose_or_panic(self.options);
        gene.add_contact_point(*to_add);
    }

    fn remove_support_mutation(&self, gene: &mut ContactPointsGene) {
        _ = gene.contact_points.remove_random(&self.rand);
    }

    fn move_support_mutation(&self, gene: &mut ContactPointsGene){
        let Some(removed) = gene.contact_points.remove_random(&self.rand) else { return };
        let Some(to_add) = self
            .graph
            .iter_adjacent(removed)
            .filter(|x| self.options_hash.contains(&x.index))
            .next() else { return };
        gene.add_contact_point(to_add.index);
    }
}

impl<'a> Mutator<ContactPointsGene, ContactPointsInitializerSettings<'a>> for ContactPointMutator<'a> {
    fn new(settings: &ContactPointsInitializerSettings<'a>, rand: Random) -> Self {
        Self {
            settings: settings.settings,
            graph: settings.graph,
            options: settings.options,
            options_hash: settings.options_hash,
            rand
        }
    }

    fn mutate(&self, gene: &mut ContactPointsGene) {
        enum MK {
            AddCp,
            RemoveCp,
            MoveCp
        }
        const OPTIONS: &[MK] = &[MK::AddCp, MK::RemoveCp, MK::MoveCp];

        let mutation = self.rand.choose_or_panic(OPTIONS);

        match mutation {
            MK::AddCp => self.add_support_mutation(gene),
            MK::RemoveCp => self.remove_support_mutation(gene),
            MK::MoveCp => self.move_support_mutation(gene)
        };
    }
}
