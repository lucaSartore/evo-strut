


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PointId(pub u32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FaceId(pub u32);

impl From<u32> for FaceId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<u32> for PointId  {
    fn from(value: u32) -> Self {
        Self(value)
    }
}


impl From<usize> for FaceId {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}

impl From<usize> for PointId  {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}

pub trait MeshId {
    fn id(&self) -> u32;
}
impl MeshId for PointId {
    fn id(&self) -> u32 {
        self.0
    }
}
impl MeshId for FaceId {
    fn id(&self) -> u32 {
        self.0
    }
}
