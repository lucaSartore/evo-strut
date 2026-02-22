use std::{collections::HashMap, rc::Rc, vec};
use rerun::external::{crossbeam::epoch::Pointable, glam::usize};
use stl_io::{IndexedMesh, IndexedTriangle};

mod settings;
pub use settings::{Settings, CriticalitySettings};

mod point;
pub use point::Point;

mod triangle;
pub use triangle::Triangle;

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

impl SurfaceNode {
    pub fn get_face(&self, graph: &SurfaceGraph) -> IndexedTriangle {
        graph.mesh.faces[self.triangle]
    }
}

pub struct SurfaceGraph {
    pub mesh: Rc<IndexedMesh>,
    pub nodes: Vec<SurfaceNode>
}


impl SurfaceGraph {
    pub fn new(mesh: &Rc<IndexedMesh>) -> Self {
        let mut to_return = Self {
            mesh: mesh.clone(),
            nodes: mesh
                .faces
                .iter()
                .enumerate()
                .map(|(i,_)| {SurfaceNode::new(i)})
                .collect()
        };
        to_return.fill_adjacent();
        to_return
    }

    pub fn get_point(&self, point: usize) -> Point {
        self.mesh.vertices[point].into()
    }

    pub fn get_triangle<'a>(&'a self, node: usize) -> Triangle<'a> {
        let t_index = self.nodes[node].triangle;
        Triangle {
            graph: self,
            index: t_index,
        }
    }

    pub fn count_triangles(&self) -> usize {
        self.mesh.faces.len()
    }
    pub fn count_vertices(&self) -> usize {
        self.mesh.vertices.len()
    }

    pub fn iter_vertices(&self) -> impl Iterator<Item=Point>{
        (0..self.count_vertices())
            .map(|x| {self.get_point(x)})
    }

    pub fn iter_triangles<'a>(&'a self) -> impl Iterator<Item=Triangle<'a>>{
        (0..self.count_triangles())
            .map(|x| {self.get_triangle(x)})
    }

    pub fn vertex_normals(&self) -> Vec<Point> {
        let mut normals = vec![Point::default(); self.count_vertices()];
        self.iter_triangles()
            .for_each(|x| {
                let raw = x.as_raw_indexed();
                for v in x.as_raw_indexed().vertices {
                    normals[v] = raw.normal.into();
                }
            });
        normals
    }


    fn fill_adjacent(&mut self) {
        let mut adj_map = HashMap::<(usize, usize), Vec<usize>>::new();
        self.nodes
            .iter()
            .enumerate()
            .for_each(|(i,n)| {
                for edge in 0..3 {
                    let triangle = &self.mesh.faces[n.triangle];
                    let side_1 = triangle.vertices[edge];
                    let side_2 = triangle.vertices[(edge+1)%3];
                    let side_identifier = if side_1 < side_2 {
                        (side_1, side_2)
                    } else {
                        (side_2, side_1) 
                    };

                    if let Some(v) = adj_map.get_mut(&side_identifier) {
                        v.push(i);
                    } else {
                        adj_map.insert(side_identifier, vec![i]);
                    }
                }
            });

        for (_, adj) in adj_map.iter() {
            for i in 0..adj.len() {
                for j in i..adj.len() {
                    self.mark_adjacent(adj[i], adj[j]);
                }
            }
        }
    }

    fn mark_adjacent(&mut self, a: usize, b: usize) {
        self.nodes[a].adjacent.push(b);
        self.nodes[b].adjacent.push(a);
    }
}


impl From<&Rc<IndexedMesh>> for SurfaceGraph {

    fn from(value: &Rc<IndexedMesh>) -> Self {
        SurfaceGraph::new(value)
    }
}
