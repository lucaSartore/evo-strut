use std::{fs::OpenOptions, rc::Rc, sync::Arc};
use anyhow::{Result, anyhow};
use super::*;
use crate::models::{Mesh, SurfaceGraph};
use baby_shark::{
    io::read_from_file, 
    mesh::corner_table::CornerTableF
};
use std::path::Path;

pub fn read(name: &str) -> Result<Mesh> {
    let Ok(mesh) = read_from_file::<CornerTableF>(Path::new(name)) else {
        return Err(anyhow!("foo"));
    };
    return Ok(mesh.into());
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
