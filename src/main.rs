mod models;
mod stages;
mod evolution;
mod support;
use std::marker::PhantomData;

use env_logger::Builder;
use log::{LevelFilter, error};

use crate::{
    evolution::TournamentBasedCrossoverSelection, models::Settings, stages::{
        OrientationBasedCriticalityDetector, Pipeline, PipelineBehaviour, StartedState, contact_point_optimization::SimpleContactPointOptimizer, criticality_grouping::DistanceBasedCriticalityGrouper
    }
};


pub trait Behaviour<TBorrow, TReturn> {
    fn run(borrow: &TBorrow) -> TReturn;
    fn generate() -> TBorrow;
}

pub trait TSettings {
    type TReturn;
    type TBorrow;
    type TBehaviour: Behaviour<Self::TBorrow, Self::TReturn>;
}

pub struct Runner<T: TSettings> {
    pd: PhantomData<T>
}
impl<T: TSettings> Runner<T> {
    pub fn run() -> T::TReturn {
        let value = T::TBehaviour::generate();
        return T::TBehaviour::run(&value);
    }
}
// impl <T: TSettings> Runner<T> {
//     fn run(
// }


fn main() {

    Builder::new()
        .filter_level(LevelFilter::Error)
        .filter_module("evo_strut", LevelFilter::Debug)
        .init();

    let settings = Settings::default();
    type Behaviour = PipelineBehaviour<
        OrientationBasedCriticalityDetector,
        DistanceBasedCriticalityGrouper,
        SimpleContactPointOptimizer
    >;
    let value = Pipeline::<StartedState, Behaviour>::run(settings);

    if let Err(e) = value {
        error!("{e:?}");
    }
}
