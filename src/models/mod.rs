use std::{collections::{HashMap, HashSet}, sync::Arc, vec};
use smallvec::{self, SmallVec};

mod settings;
pub use settings::{Settings, CriticalitySettings, IoSettings, ContactPointsOptimizationSettings};

mod point;
pub use point::Point;

mod triangle;
pub use triangle::Triangle;

mod ids;
pub use ids::{PointId, FaceId, MeshId};

mod mesh;
pub use mesh::{Mesh, Face};

mod mesh_vector;
pub use mesh_vector::MeshVector;

mod plane;
pub use plane::Plane;


#[derive(Debug, Clone)]
pub struct SurfaceNode {
    pub face: FaceId,
    pub neighbors: SmallVec<[FaceId; 3]>
}


impl SurfaceNode {
    pub fn new(face: FaceId) -> Self {
        Self {
            face,
            neighbors: SmallVec::new()
        }
    }
}

impl SurfaceNode {
    pub fn get_face(&self, graph: &SurfaceGraph) -> Face {
        graph.mesh.faces[self.face]
    }
}

pub struct SurfaceGraph {
    pub mesh: Arc<Mesh>,
    pub nodes: MeshVector<FaceId, SurfaceNode>
}


impl SurfaceGraph {
    pub fn new(mesh: &Arc<Mesh>) -> Self {
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
        self.mesh.points[point]
    }

    pub fn get_node(& self, node: FaceId) -> &SurfaceNode {
        &self.nodes[node]
    }

    pub fn get_triangle<'a>(&'a self, node: FaceId) -> Triangle<'a> {
        let f_index = self.nodes[node].face;
        Triangle {
            graph: self,
            index: f_index,
        }
    }

    pub fn count_triangles(&self) -> usize {
        self.mesh.faces.len()
    }
    pub fn count_vertices(&self) -> usize {
        self.mesh.points.len()
    }

    pub fn iter_adjacent<'a>(&'a self, node: FaceId) -> impl Iterator<Item=Triangle<'a>>{
        self.nodes[node]
            .neighbors
            .iter()
            .map(|x| {
                self.get_triangle(*x)
            })
    }
    pub fn iter_vertices(&self) -> impl Iterator<Item=Point>{
        (0..self.count_vertices())
            .map(|x| {self.get_point(x.into())})
    }

    pub fn iter_triangles<'a>(&'a self, filter: Option<&HashSet<FaceId>>) -> impl Iterator<Item=Triangle<'a>>{
        let mut v: Vec<_> = (0..self.count_triangles())
            .map(|x| {self.get_triangle(x.into())})
            .collect();

        if let Some(set) = filter {
            v.retain(|x| set.contains(&x.index));
        }

        v.into_iter()
    }

    pub fn vertex_normals(&self, filter: Option<&HashSet<FaceId>>) -> Vec<Point> {
        let mut normals = vec![Point::default(); self.count_vertices()];
        self.iter_triangles(filter)
            .for_each(|x| {
                let raw = x.as_raw_indexed();
                for v in x.as_raw_indexed().vertexes {
                    normals[v.index()] = raw.normal;
                }
            });
        normals
    }


    fn fill_adjacent(&mut self) {
        // for each couple of connected points
        // we map a list of the triangles that are inside
        let mut adj_map = HashMap::<(PointId, PointId), Vec<FaceId>>::new();
        self.nodes
            .iter()
            .enumerate()
            .for_each(|(i,n)| {
                for edge in 0..3 {
                    let triangle = &self.mesh.faces[n.face];
                    let side_1 = triangle.vertexes[edge];
                    let side_2 = triangle.vertexes[(edge+1)%3];
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
                for j in i+1..adj.len() {
                    self.mark_adjacent(adj[i], adj[j]);
                }
            }
        }
    }

    fn mark_adjacent(&mut self, a: FaceId, b: FaceId) {
        self.nodes[a].neighbors.push(b);
        self.nodes[b].neighbors.push(a);
    }

    pub fn neighbors(&self, t: FaceId) -> SmallVec<[FaceId; 3]> {
        self.get_node(t).neighbors.clone()
    }

}
