use std::{collections::HashMap, rc::Rc, sync::Arc, vec};
use rerun::external::{crossbeam::epoch::Pointable, glam::usize};
use stl_io::{IndexedMesh, IndexedTriangle};

mod settings;
pub use settings::{Settings, CriticalitySettings};

mod point;
pub use point::Point;

mod triangle;
pub use triangle::Triangle;

pub mod ids;
pub use ids::{PointId, TriangleId};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct SurfaceNode {
    pub triangle: TriangleId,
    pub adjacent: Vec<TriangleId>
}

impl SurfaceNode {
    pub fn new(triangle: TriangleId) -> Self {
        Self {
            triangle,
            adjacent: Vec::new()
        }
    }
}

impl SurfaceNode {
    pub fn get_face(&self, graph: &SurfaceGraph) -> IndexedTriangle {
        graph.mesh.faces[self.triangle.0]
    }
}

pub struct SurfaceGraph {
    pub mesh: Arc<IndexedMesh>,
    pub nodes: Vec<SurfaceNode>
}


impl SurfaceGraph {
    pub fn new(mesh: &Arc<IndexedMesh>) -> Self {
        let mut to_return = Self {
            mesh: mesh.clone(),
            nodes: mesh
                .faces
                .iter()
                .enumerate()
                .map(|(i,_)| {SurfaceNode::new(i.into())})
                .collect()
        };
        to_return.fill_adjacent();
        to_return
    }

    pub fn get_point(&self, point: PointId) -> Point {
        self.mesh.vertices[point.0].into()
    }

    pub fn get_node(& self, node: TriangleId) -> &SurfaceNode {
        &self.nodes[node.0]
    }

    pub fn get_triangle<'a>(&'a self, node: TriangleId) -> Triangle<'a> {
        let t_index = self.nodes[node.0].triangle;
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

    pub fn iter_adjacent<'a>(&'a self, node: TriangleId) -> impl Iterator<Item=Triangle<'a>>{
        self.nodes[node.0]
            .adjacent
            .iter()
            .map(|x| {
                self.get_triangle(*x)
            })
    }
    pub fn iter_vertices(&self) -> impl Iterator<Item=Point>{
        (0..self.count_vertices())
            .map(|x| {self.get_point(x.into())})
    }

    pub fn iter_triangles<'a>(&'a self) -> impl Iterator<Item=Triangle<'a>>{
        (0..self.count_triangles())
            .map(|x| {self.get_triangle(x.into())})
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
        // for each couple of connected points
        // we map a list of the triangles that are inside
        let mut adj_map = HashMap::<(PointId, PointId), Vec<TriangleId>>::new();
        self.nodes
            .iter()
            .enumerate()
            .for_each(|(i,n)| {
                for edge in 0..3 {
                    let triangle = &self.mesh.faces[n.triangle.0];
                    let side_1 = triangle.vertices[edge];
                    let side_2 = triangle.vertices[(edge+1)%3];
                    let side_identifier: (PointId, PointId) = if side_1 < side_2 {
                        (side_1.into(), side_2.into())
                    } else {
                        (side_2.into(), side_1.into()) 
                    };

                    if let Some(v) = adj_map.get_mut(&side_identifier) {
                        v.push(i.into());
                    } else {
                        adj_map.insert(side_identifier, vec![i.into()]);
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

    fn mark_adjacent(&mut self, a: TriangleId, b: TriangleId) {
        self.nodes[a.0].adjacent.push(b);
        self.nodes[b.0].adjacent.push(a);
    }
}


impl From<&Arc<IndexedMesh>> for SurfaceGraph {

    fn from(value: &Arc<IndexedMesh>) -> Self {
        SurfaceGraph::new(value)
    }
}
