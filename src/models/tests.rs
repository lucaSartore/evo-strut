use stl_io::Vector;

use super::*;

#[test]
fn test_is_point_inside() {
    // one right angle tirangle
    // V1(0,0), V2(10,0), V3(0,10)
    let mesh = IndexedMesh {
        vertices: vec![
            Vector([0.,0.,0.]),
            Vector([10.,0.,0.]),
            Vector([0.,10.,0.])
        ],
        faces: vec![
            IndexedTriangle {
                normal: Vector([0.,0.,1.]),
                vertices: [0,1,2]
            }
        ]
    };

    let g = SurfaceGraph::new(&Arc::new(mesh));
    let triangle = g.get_triangle(TriangleId(0));

    // 1. Point clearly inside
    assert!(triangle.is_point_inside_footprint(Point { x: 1.0, y: 1.0, z: 0. }));
    assert!(triangle.is_point_inside_footprint(Point { x: 3.0, y: 3.0, z: 0. }));

    // 2. Point clearly outside
    assert!(!triangle.is_point_inside_footprint(Point { x: 11.0, y: 0.0, z: 0. }));
    assert!(!triangle.is_point_inside_footprint(Point { x: -1.0, y: -1.0, z: 0. }));
    assert!(!triangle.is_point_inside_footprint(Point { x: 6.0, y: 6.0, z: 0. }));

    // 3. Points on the edges
    assert!(triangle.is_point_inside_footprint(Point { x: 0.0, y: 5.0, z: 0. }));
    assert!(triangle.is_point_inside_footprint(Point { x: 5.0, y: 0.0, z: 0. }));

    // 4. Points on the vertices
    assert!(triangle.is_point_inside_footprint(Point { x: 0.0, y: 0.0, z: 0. }));
    assert!(triangle.is_point_inside_footprint(Point { x: 10.0, y: 0.0, z: 0. }));
}
