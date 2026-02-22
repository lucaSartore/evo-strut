use stl_io::Vector;
use rerun::Position3D;

pub struct Point(pub [f32;3]);


impl From<&Vector<f32>> for Point {
    fn from(value: &Vector<f32>) -> Self {
        return Point(value.0)
    }
}

impl From<Point> for Position3D {
    fn from(value: Point) -> Self {
        return Position3D::new(value.0[0], value.0[1], value.0[2])
    }
}
