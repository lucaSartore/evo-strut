use super::SupportGroup;
use crate::{
    evolution::{Crossover, Random},
    models::Settings,
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

impl<'a> Crossover<SupportGroup, SupportStructureCrossoverSettings<'a>>
    for SupportStructureCrossover<'a>
{
    fn new(settings: &SupportStructureCrossoverSettings<'a>, rand: Random) -> Self {
        Self {
            settings: settings.settings,
            rand,
        }
    }

    fn crossover(&self, a: &SupportGroup, _b: &SupportGroup) -> SupportGroup {
        // todo: implement real crossover
        a.clone()
    }
}
