use std::marker::PhantomData;
use super::*;
use std::ops::{Index, IndexMut};

pub struct MeshVector<K, V> 
where 
    K: MeshId 
{
    _d: PhantomData<K>,
    data: Vec<V>,
}

impl<K, V> MeshVector<K, V> 
where 
    K: MeshId 
{
    pub fn new() -> Self {
        Self {
            _d: PhantomData,
            data: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            _d: PhantomData,
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn push(&mut self, value: V) {
        self.data.push(value);
    }

    pub fn get(&self, id: K) -> Option<&V> {
        self.data.get(id.id() as usize)
    }

    pub fn get_mut(&mut self, id: K) -> Option<&mut V> {
        self.data.get_mut(id.id() as usize)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, V> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, V> {
        self.data.iter_mut()
    }
}

impl<K, V> Index<K> for MeshVector<K, V> 
where 
    K: MeshId 
{
    type Output = V;
    fn index(&self, id: K) -> &Self::Output {
        self.get(id).expect("MeshVector: ID not found")
    }
}

impl<K, V> IndexMut<K> for MeshVector<K, V> 
where 
    K: MeshId 
{
    fn index_mut(&mut self, id: K) -> &mut Self::Output {
        self.get_mut(id).expect("MeshVector: ID not found")
    }
}

impl<'a, K, V> IntoIterator for &'a MeshVector<K, V> 
where 
    K: MeshId 
{
    type Item = &'a V;
    type IntoIter = std::slice::Iter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<K, V> FromIterator<V> for MeshVector<K, V>
where
    K: MeshId,
{
    fn from_iter<I: IntoIterator<Item = V>>(iter: I) -> Self {
        Self {
            _d: PhantomData::default(),
            data: iter.into_iter().collect()
        }
    }
}
