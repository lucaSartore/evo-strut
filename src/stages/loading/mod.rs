use super::*;
use crate::models::{IoSettings, SurfaceGraph};
use anyhow::{anyhow, Result};
use baby_shark::{
    io::{read_from_file, write_to_file},
    mesh::corner_table::CornerTableF,
    remeshing::incremental::IncrementalRemesher,
};
use std::sync::Arc;
use std::{fs::File, io::BufReader, path::Path};

pub fn read(name: &str) -> Result<CornerTableF> {
    let r = read_from_file::<CornerTableF>(Path::new(name));
    let mesh = match r {
        Ok(m) => m,
        Err(e) => return Err(anyhow!("error while loading file \"{}\"\n{:?}", name, e)),
    };
    Ok(mesh)
}

pub struct LoadingStage<TB>
where
    TB: PipelineBehaviourTrait,
{
    _b: PhantomData<TB>,
}

impl<TB> LoadingStage<TB>
where
    TB: PipelineBehaviourTrait,
{
    fn remesh(mut mesh: CornerTableF, settings: &IoSettings) -> Result<CornerTableF> {
        let remesher = IncrementalRemesher::default();

        if settings.target_edge_length != 0. {
            remesher.remesh(&mut mesh, settings.target_edge_length);
        }
        Ok(mesh)
    }
}

pub enum LoadingStageOutput<TB>
where
    TB: PipelineBehaviourTrait,
{
    MeshLoaded(Pipeline<LoadedState, TB>),
    StructureLoaded(Pipeline<SupportStructureOptimizedState, TB>),
}

impl<TB> LoadingStage<TB>
where
    TB: PipelineBehaviourTrait,
{
    pub fn execute(input: Pipeline<StartedState, TB>) -> Result<LoadingStageOutput<TB>> {
        let settings = &input.state.settings.io_settings;
        let mesh = read(&settings.input_file_path)?;
        let mesh = Self::remesh(mesh, settings)?;

        if let Some(path) = &settings.re_meshed_input_file_path {
            let r = write_to_file(&mesh, Path::new(path));
            if let Err(e) = r {
                return Err(anyhow!(
                    "error while loading writing file \"{}\"\n{:?}",
                    path,
                    e
                ));
            };
        }

        let mesh_rc = Arc::new(mesh.into());

        let graph = SurfaceGraph::new(&mesh_rc);

        let Some(path) = settings.input_json_path.as_ref() else {
            let state = LoadedState {
                settings: input.state.settings,
                graph,
            };
            let pipeline = Pipeline::from_state(state);
            return Ok(LoadingStageOutput::MeshLoaded(pipeline));
        };

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let state = SupportStructureOptimizedState {
            settings: input.state.settings,
            graph,
            connection_points: ContactPointsGene::default(),
            support_structures: serde_json::from_reader(reader)?,
        };
        let pipeline = Pipeline::from_state(state);

        return Ok(LoadingStageOutput::StructureLoaded(pipeline));
    }
}
