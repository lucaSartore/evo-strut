use std::sync::Arc;

use baby_shark::mesh::corner_table::CornerTableF;

use crate::{
    models::{Point, SurfaceGraph},
    stages::visualization::visualize_mesh,
};

use super::*;

#[test]
fn test() {
    let shape = Sphere::new(Point::new(10., 20., 30.), 10.);
    let mesh = ShapeGenerator::<CornerTableF>::build(&shape, 1.).unwrap();
    let mesh = Arc::new(mesh.into());
    let mesh = SurfaceGraph::new(&mesh);
    visualize_mesh(&mesh, "test", None).unwrap();
}
