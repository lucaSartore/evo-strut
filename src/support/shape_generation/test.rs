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
    let mut converter = MeshToVolume::default().with_voxel_size(0.1);
    converter.convert(&mesh);
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


pub fn test_composition() {
    let mut builder = ShapeFactory::new();
    let mut settings = Settings::default();
    let voxel_size = 0.1;
    settings.support_settings.voxel_size = voxel_size;

    let sphere = Sphere::new(
        Point::new(0., 0., 0.),
        10.
    );

    let cone = TruncatedCone::new(
        Circle::new(
            Point::new(0., 0., 0.),
            10.,
            Point::UPWARD
        ),
        Circle::new(
            Point::new(0., 0., 40.),
            30.,
            Point::UPWARD
        ),
    );
    let cone_neg = TruncatedCone::new(
        Circle::new(
            Point::new(0., 0., 0.),
            0.1,
            Point::UPWARD
        ),
        Circle::new(
            Point::new(0., 0., 40.),
            29.,
            Point::UPWARD
        ),
    );

    builder.add_positive_shape(sphere);
    builder.add_positive_shape(cone);
    builder.add_negative_shape(cone_neg);
    let mesh = builder.build(&settings).unwrap();
    let mesh_rc = Arc::new(mesh.into());
    let graph = SurfaceGraph::new(&mesh_rc);

    visualize_mesh(&graph, "merged mesh", None).unwrap();
}
