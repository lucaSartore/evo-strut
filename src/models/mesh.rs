use baby_shark::{mesh::{corner_table::CornerTableF, traits::{TriangleMesh, Triangles}}};
use baby_shark::io::*;
use itertools::Itertools;
use super::*;


#[derive(Debug, Clone, Copy)]
pub struct Face {
    pub vertexes: [PointId; 3],
    pub normal: Point
}


pub struct Mesh {
    pub points: MeshVector<PointId, Point>,
    pub faces: MeshVector<FaceId, Face>
}

impl From<CornerTableF> for Mesh {
    fn from(value: CornerTableF) -> Self {
        let points: MeshVector<PointId, _> = value
            .vertices()
            .sorted()
            .map(|v| {
                let [x,y,z] = value.position(v);
                Point {x,y,z}
            })
            .collect();

        let hashmap: HashMap<Point, PointId> = points
            .iter()
            .enumerate()
            .map(|(i,p)| (*p, PointId(i as u32)))
            .collect();

        let faces = value
            .triangles()
            .map(|t| {
                let n = t.get_normal()
                    .expect("Got malformed triangle, that does not have valid surface normal");
                let (p1, p2, p3) = (*t.p1(), *t.p2(), *t.p3());
                Face{
                    vertexes: [
                        hashmap[&p1.into()],
                        hashmap[&p2.into()],
                        hashmap[&p3.into()]
                    ],
                    normal: n.into()
                }
            })
            .collect();
        
        Mesh { 
            points,
            faces
        }
    }
}


impl From<Mesh> for CornerTableF {
    fn from(val: Mesh) -> Self {
        let mut builder = CornerTableF::builder_soup();
        builder.set_num_faces(val.faces.len());
        for face in val.faces.iter() {
            let [v1,v2,v3] = face.vertexes;
            let p1: [f32;3] = val.points[v1].into();
            let p2: [f32;3] = val.points[v2].into();
            let p3: [f32;3] = val.points[v3].into();
            builder.add_face(p1,p2,p3)
                .expect("error in mesh creation");
        }
        builder.finish()
            .expect("error in mesh creation")
    }
}
