use std::{fs::OpenOptions, rc::Rc, sync::Arc};
use stl_io::IndexedMesh;
use anyhow::Result;
use super::*;
use crate::models::SurfaceGraph;

pub fn read(name: &str) -> Result<IndexedMesh> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(name)
        .unwrap();
    Ok(stl_io::read_stl(&mut file)?)
}

pub struct LoadingStage<TB> 
where
    TB: PipelineBehaviourTrait,
{
    _b: PhantomData<TB>
}

impl<TB> LoadingStage<TB>
where 
    TB: PipelineBehaviourTrait,
{
    pub fn execute(
        input: Pipeline<StartedState, TB>
    ) -> Result<Pipeline<LoadedState, TB>> {
        let mesh = read(&input.state.settings.io_settings.input_file_path)?;
        let mesh_rc = Arc::new(mesh);
        let graph = SurfaceGraph::new(&mesh_rc);
        let state = LoadedState {
            settings: input.state.settings,
            graph
        };
        let pipeline = Pipeline::from_state(state);
        Ok(pipeline)
    }
}
