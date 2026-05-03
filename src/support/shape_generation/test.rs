use std::sync::Arc;

use baby_shark::mesh::corner_table::CornerTableF;

use crate::{
    models::{Point, SurfaceGraph},
    stages::visualization::visualize_mesh,
};

use super::*;

#[test]
fn test_sphere() {
    let shape = Sphere::new(Point::new(10., 20., 30.), 10.);
    let mesh = ShapeGenerator::<CornerTableF>::build(&shape, 1.).unwrap();
    let mesh = Arc::new(mesh.into());
    let mesh = SurfaceGraph::new(&mesh);
    visualize_mesh(&mesh, "test sphere", None).unwrap();
}

#[test]
fn test_cone_1() {
    let p1 = Point::new(10., 10., 10.);
    let p2 = Point::new(20., 20., 50.);
    let v = (p2 - p1).as_versor();
    let r = 5.;
    let shape = TruncatedCone::new(Circle::new(p1, r, v), Circle::new(p2, r, v));
    let mesh = ShapeGenerator::<CornerTableF>::build(&shape, 1.).unwrap();
    let mesh = Arc::new(mesh.into());
    let mesh = SurfaceGraph::new(&mesh);
    visualize_mesh(&mesh, "test cone 1", None).unwrap();
}

#[test]
fn test_cone_2() {
    let p1 = Point::new(10., 10., 10.);
    let p2 = Point::new(20., 20., 50.);
    let v = (p2 - p1).as_versor();
    let shape = TruncatedCone::new(Circle::new(p1, 5., v), Circle::new(p2, 40., Point::UPWARD));
    let mesh = ShapeGenerator::<CornerTableF>::build(&shape, 1.).unwrap();
    let mesh = Arc::new(mesh.into());
    let mesh = SurfaceGraph::new(&mesh);
    visualize_mesh(&mesh, "test cone 2", None).unwrap();
}

