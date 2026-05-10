use std::{fmt::Debug, marker::PhantomData};

use anyhow::anyhow;
use baby_shark::{geometry::{primitives::triangle3::Triangle3}, io::IndexedBuilder};
use hashbrown::HashMap;

use crate::models::Point;


/// utility structure used to build CONVEX polygons
/// the structure automatically ensures that the faces
/// are paced facing outward.
pub struct BuilderWrapper<TBuilder, TM> 
where
    TBuilder: IndexedBuilder<f32, TM>,
{
    _marker: PhantomData<TM>,
    builder: TBuilder,
    positions: HashMap<usize, Point>,
    polygon_center: Point
}

fn map_err<TV, TE>(value: std::result::Result<TV, TE>) -> anyhow::Result<TV>
where
    TE: Debug,
{
    value.map_err(|x| anyhow!("Error in mesh creation: {x:?}"))
}

impl<TBuilder, TM> BuilderWrapper<TBuilder, TM>
where
    TBuilder: IndexedBuilder<f32, TM>,
{
    pub fn new(builder: TBuilder, polygon_center: Point) -> Self
    {
        Self {
            _marker: Default::default(),
            builder,
            polygon_center,
            positions: HashMap::default()
        }
    }

    pub fn add_vertex(&mut self, v: Point) -> anyhow::Result<usize> {
        let index = map_err(self.builder.add_vertex(v))?;
        self.positions.insert(index, v);
        anyhow::Ok(index)
    }

    pub fn add_face(&mut self, a: usize, mut b: usize, mut c: usize) -> anyhow::Result<()> {
        let pa = self.positions[&a];
        let pb = self.positions[&b];
        let pc = self.positions[&c];

        let t = Triangle3::new(pa.into(), pb.into(), pc.into());

        let triangle_center: Point = t.center().into();
        let triangle_normal: Point = t.get_normal().expect("normal should always exist").into();
        let triangle_normal = triangle_normal.as_versor();
        let actual_normal = (triangle_center - self.polygon_center).as_versor();
        let dot = Point::dot(triangle_normal, actual_normal);
        if dot < 0. {
            (b, c) = (c, b)
        }

        map_err(self.builder.add_face(a, b, c))
    }

    pub fn finish(self) -> anyhow::Result<TM> {
        map_err(self.builder.finish())
    }
}
