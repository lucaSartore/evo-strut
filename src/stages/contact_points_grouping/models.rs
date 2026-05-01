use anyhow::Result;
use hashbrown::HashMap;
use rand::random;
use std::fmt::Debug;

use crate::{
    evolution::{Cost, Random},
    models::{Settings, SurfaceGraph},
    stages::{
        contact_point_optimization::ContactPointsGene,
        support_structure_optimization::{
            mutation::SupportStructureMutator, CompressedSupportGene, ContactPoint, SupportGroup,
        },
        support_structure_refinement::SupportStructureGene,
    },
    support::neural_network::NeuralNetwork,
};

#[derive(Clone, Debug)]
pub struct ContactPointGroupingGene {
    pub network: NeuralNetwork,
}

impl ContactPointGroupingGene {

    pub fn to_compressed_gene(
        &self,
        points: &ContactPointsGene,
        graph: &SurfaceGraph,
        settings: &Settings,
        rand: &Random,
    ) -> CompressedSupportGene {
        let mut grouped = HashMap::<usize, Vec<_>>::default();

        for contact in points.iter_contacts() {
            let position = graph.get_triangle(*contact.0).center();
            let values = self
                .network
                .evaluate(&[position.x, position.y])
                .expect("network has failed to return a valid result");
            let group = values
                .into_iter()
                .enumerate()
                .max_by_key(|(_i, v)| Cost::new(*v))
                .expect("network can't have zero output layers")
                .0;

            let cp = ContactPoint {
                position,
                radius: contact.1.radius,
            };
            if let Some(e) = grouped.get_mut(&group) {
                e.push(cp);
            } else {
                grouped.insert(group, vec![cp]);
            }
        }

        let mutator = SupportStructureMutator {
            settings,
            graph,
            rand: rand.seeded_copy(),
        };

        CompressedSupportGene {
            groups: grouped
                .into_values()
                .map(|x| {
                    let mut g = SupportGroup::from_supports(x);
                    g.regenerate(&mutator);
                    return g;
                })
                .collect(),
        }
    }

    pub fn to_full_gene(
        &self,
        points: &ContactPointsGene,
        graph: &SurfaceGraph,
        settings: &Settings,
        rand: &Random,
    ) -> SupportStructureGene {
        self.to_compressed_gene(points, graph, settings, rand).to_full_gene(graph)
    }

    pub fn new(network: NeuralNetwork) -> Self {
        Self { network }
    }
}
