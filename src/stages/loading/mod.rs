use std::sync::Arc;
use anyhow::{Result, anyhow};
use super::*;
use crate::models::{IoSettings, Mesh, SurfaceGraph};
use baby_shark::{
    io::read_from_file, 
    mesh::corner_table::CornerTableF, remeshing::{self, voxel::VoxelRemesher}
};
use std::path::Path;

pub fn read(name: &str) -> Result<CornerTableF> {
    let r = read_from_file::<CornerTableF>(Path::new(name));
    let mesh = match r {
        Ok(m) => m,
        Err(e) =>  return Err(anyhow!("error while loading file \"{}\"\n{:?}", name, e))
    };
    Ok(mesh)
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
    fn remesh(mesh: CornerTableF, settings: &IoSettings) -> Result<CornerTableF> {
        let mut remesher = VoxelRemesher::default()
            .with_voxel_size(settings.voxel_size);
        let mesh = match remesher.remesh(&mesh) {
            Some(m) => m,
            None => return Err(anyhow!("fail to execute re-meshing"))
        };
        Ok(mesh)
    }
    
}



impl<TB> LoadingStage<TB>
where 
    TB: PipelineBehaviourTrait,
{
    pub fn execute(
        input: Pipeline<StartedState, TB>
    ) -> Result<Pipeline<LoadedState, TB>> {
        let settings = &input.state.settings.io_settings;
        let mesh = read(&settings.input_file_path)?;
        let mesh = Self::remesh(mesh, settings)?;
        let mesh_rc = Arc::new(mesh.into());

        let graph = SurfaceGraph::new(&mesh_rc);
        let state = LoadedState {
            settings: input.state.settings,
            graph
        };
        let pipeline = Pipeline::from_state(state);
        Ok(pipeline)
    }
}
