use crate::stages::support_structure_refinement::SupportStructureGene;


#[derive(Clone, Debug)]
pub struct CompressedSupportGene {
}

impl CompressedSupportGene {
    pub fn to_full_genes(&self) -> Vec<SupportStructureGene> {
        todo!();
    }
    pub fn to_full_gene(&self) -> SupportStructureGene {
        todo!();
    }
}
