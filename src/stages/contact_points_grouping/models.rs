use hashbrown::HashMap;
use serde::Serialize;
use std::fmt::Debug;

use crate::{
    evolution::{Cost, Random},
    models::{Settings, SurfaceGraph},
    stages::{
        contact_point_optimization::ContactPointsGene,
        support_structure_optimization::{
            ContactPoint, SupportStructureOptimizationGene, mutation::SupportStructureMutator,
        },
    },
    support::neural_network::NeuralNetwork,
};

#[derive(Clone, Debug, Serialize)]
pub struct ContactPointGroupingGene {
    pub network: NeuralNetwork,
}

impl ContactPointGroupingGene {
    pub fn to_groups(
        &self,
        points: &ContactPointsGene,
        graph: &SurfaceGraph,
        settings: &Settings,
        rand: &Random,
    ) -> Vec<SupportStructureOptimizationGene> {
        let mut grouped = HashMap::<usize, Vec<_>>::default();

        for contact in points.iter_contacts() {
            let position = graph.get_triangle(*contact.0).center();
            let values = self
                .network
                .evaluate(&[position.x, position.y, position.z])
                .expect("network has failed to return a valid result");
            let group = values
                .into_iter()
                .enumerate()
                .max_by_key(|(_i, v)| Cost::new(*v))
                .expect("network can't have zero output layers")
                .0;

            let cp = ContactPoint {
                face: *contact.0,
                position,
                radius: contact.1.radius,
            };
            if let Some(e) = grouped.get_mut(&group) {
                e.push(cp);
            } else {
                grouped.insert(group, vec![cp]);
            }
        }

        grouped
            .into_values()
            .map(|x| SupportStructureOptimizationGene::from_contacts(x, rand))
            .collect()
    }

    pub fn new(network: NeuralNetwork) -> Self {
        Self { network }
    }
}
