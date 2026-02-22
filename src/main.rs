mod stages;
mod models;
use anyhow::Result;

use std::rc::Rc;

use models::SurfaceGraph;
use stages::visualization::{visualize_mesh, Color};

fn main() -> Result<()> {
    let mesh = stages::loading::read("test_meshes/dragon.stl")?;
    let mesh_rc = Rc::new(mesh);
    
    let graph = SurfaceGraph::new(&mesh_rc);

    visualize_mesh(graph, "foo", Color::Red)?;
    
    Ok(())
}
