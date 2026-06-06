use crate::{
    models::Point,
    stages::support_structure_optimization::{SupportPoint, models::SupportStructureOptimizationGene},
    support::random_distribution::RandomDistribution,
};

use super::*;

pub fn mutate(mutator: &SupportStructureMutator, gene: &mut SupportStructureOptimizationGene) {
    let rand = &mutator.rand;
    let strategy_picker = rand.next_distribution(&RandomDistribution::InRange { low: 0.0, high: 1.0 });

    let position = if strategy_picker < 0.33 {
        // Standard DE exploration
        let p1 = gene.random_point(rand);
        let p2 = gene.random_point(rand);
        let p3 = gene.random_point(rand);
        let f = rand.next_distribution(&RandomDistribution::InRange { low: 0.4, high: 0.9 });
        p1 + (p2 - p3).to_scaled(f)
        
    } else if strategy_picker < 0.67 {
        // Truss/Midpoint building
        let p1 = gene.random_point(rand);
        let p2 = gene.random_point_close_to(p1, 0.2, 30.0_f32.to_radians(), rand);
        let scaler = rand.next_f32(0., 1.);
        let midpoint = p1 + (p2 - p1).to_scaled(scaler);
        Point::random(midpoint, 0.5, rand)
        
    } else {
        // Ground Anchoring
        let p1 = gene.random_point(rand);
        let mut p = p1;
        p.z *= rand.next_distribution(&RandomDistribution::InRange { low: 0.0, high: 1.0 });
        p
    };

    let num_contacts = rand.next_u32() % 3 + 1;
    // todo: hardcoded values
    let radius = rand.next_f32(1., 5.);

    gene.supports.push(SupportPoint { position, num_contacts, radius });
}

