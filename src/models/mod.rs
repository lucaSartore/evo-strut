use std::rc::Rc;
use stl_io::IndexedMesh;

mod point;
pub use point::Point;

pub struct SurfaceNode {
    pub triangle: usize,
    pub adjacent: Vec<usize>
}

impl SurfaceNode {
    pub fn new(triangle: usize) -> Self {
        Self {
            triangle,
            adjacent: Vec::new()
        }
    }
}

pub struct SurfaceGraph {
    pub mesh: Rc<IndexedMesh>,
    pub nodes: Vec<SurfaceNode>
    
}


impl SurfaceGraph {
    pub fn new(mesh: &Rc<IndexedMesh>) -> Self {
        Self {
            mesh: mesh.clone(),
            nodes: mesh
                .faces
                .iter()
                .enumerate()
                .map(|(i,_)| {SurfaceNode::new(i)})
                .collect()
        }
    }

    fn fill_adjacent(self) {
    }
}


impl From<&Rc<IndexedMesh>> for SurfaceGraph {

    fn from(value: &Rc<IndexedMesh>) -> Self {
        return SurfaceGraph::new(value)
    }
}
