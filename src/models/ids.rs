#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TriangleId(pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PointId(pub usize);

impl From<usize> for TriangleId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<usize> for PointId  {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
