use hashbrown::HashSet;

use crate::{evolution::{Mutator, Random}, models::{FaceId, Settings, SurfaceGraph}, stages::contact_point_optimization::{ContactPointShape, initializer::ContactPointsInitializerSettings}, support::{graph_circle::find_circle, remove_random::RemoveRandom}};
use super::models::ContactPointsGene;

pub struct ContactPointsMutatorSettings<'a> {
    pub settings: &'a Settings,
    pub graph: &'a SurfaceGraph,
    pub options: &'a [FaceId],
    pub options_hash: &'a HashSet<FaceId>
}

impl<'a> ContactPointsMutatorSettings<'a> {
    pub fn new(settings: &'a Settings, graph: &'a SurfaceGraph, options: &'a [FaceId], options_hash: &'a HashSet<FaceId>) -> Self {
        Self {
            settings,
            graph,
            options,
            options_hash
        }
    }
    
}

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
        gene.add_contact_point(*to_add, ContactPointShape::random(&self.rand, self.settings));
    }

    fn remove_support_mutation(&self, gene: &mut ContactPointsGene) {
        _ = gene.contact_points.remove_random(&self.rand);
    }

    fn move_support_mutation(&self, gene: &mut ContactPointsGene){
        let Some(removed) = gene.contact_points.remove_random(&self.rand) else { return };
        let radius = self.settings.contact_points_optimization_settings.move_support_mutation_intensity;
        let mut options = find_circle(self.graph, removed.0, radius, false);
        options = options.intersection(self.options_hash).copied().collect();

        let selected = options.choose_random(&self.rand)
            .expect("random selection failed");
        gene.add_contact_point(*selected, removed.1);
    }

    fn change_support_size_mutation(&self, gene: &mut ContactPointsGene){
        let Some(to_edit) = gene.contact_points.choose_random(&self.rand) else { return };
        let to_edit = to_edit.0;
        let Some(shape) = gene.contact_points.get_mut(&to_edit) else { return };
        shape.mutate(&self.rand, self.settings);
    }
}

impl<'a> Mutator<ContactPointsGene, ContactPointsMutatorSettings<'a>> for ContactPointMutator<'a> {
    fn new(settings: &ContactPointsMutatorSettings<'a>, rand: Random) -> Self {
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
            MoveCp,
            ChangeSize,
        }
        const OPTIONS: &[MK] = &[MK::AddCp, MK::RemoveCp, MK::MoveCp, MK::ChangeSize];

        let mutation = self.rand.choose_or_panic(OPTIONS);

        match mutation {
            MK::AddCp => self.add_support_mutation(gene),
            MK::RemoveCp => self.remove_support_mutation(gene),
            MK::MoveCp => self.move_support_mutation(gene),
            MK::ChangeSize => self.change_support_size_mutation(gene),
        };
    }
}
