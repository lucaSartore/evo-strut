use crate::models::SurfaceGraph;
use super::*;


/// The genome generation phases essentially consist in the transformation
/// of a `SupportStructureGene` into a `SupportGraph`.
/// The first one is a "compressed" representation that is ideal for operating
/// crossovers and mutations, while the latter one is a "verbose" representation
/// that is much more suited for evaluation.
pub struct GenomeGenerator<'a> {
    graph: &'a SurfaceGraph
}

impl<'a> GenomeGenerator<'a> {
    pub fn generate(&self, genome: &SupportStructureGenome) -> SupportGraph {
        todo!()
    }
}
