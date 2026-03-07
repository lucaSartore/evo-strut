use std::{collections::{HashMap, HashSet}, ops::Add};

use rerun::external::glam::usize;



/// represent a unique identifier of one contact point that the
/// algorithm decided to place
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub  struct ContactIdentifier(usize);

impl Add for ContactIdentifier {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

/// represent a connection between two contact points
/// connected points form a bridge that sustain all the
/// structure in between
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub struct Connection {
    a: ContactIdentifier,
    b: ContactIdentifier
}

impl Connection {
    pub fn new(mut a: ContactIdentifier, mut b: ContactIdentifier) -> Self {
        assert!(a != b);
        if a > b {
            (a,b) = (b,a);
        }
        Self{a,b}
    }

    pub fn shift_ids(&mut self, offset: ContactIdentifier) {
        self.a = self.a + offset;
        self.b = self.b + offset;
    }
}

/// represent the position of one contact point
#[derive(Debug, Clone)]
pub struct ContactPoint {
    pub identifier: ContactIdentifier,
    pub x: f32,
    pub y: f32
}

impl ContactPoint {
    pub fn shift_ids(&mut self, offset: ContactIdentifier) {
        self.identifier = self.identifier + offset
    }
}

/// uniquely identify one support structure that can be mutated, evaluated ecc.
#[derive(Debug, Clone, Default)]
pub struct ContactPointsGene {
    points: HashMap<ContactIdentifier, ContactPoint>,
    connections: HashSet<Connection>
}

impl ContactPointsGene {
    pub fn merge_many(elements: Vec<Self>) -> Option<Self> {
        elements.into_iter().reduce(|mut acc, e| {
            acc.merge_with(e);
            return acc;
        })
    }
    pub fn merge_with(&mut self, other: Self) {
        let offset = self.points.keys().max()
            .map_or(
                ContactIdentifier(0),
                |x| *x + ContactIdentifier(1)
            );
        other.points.into_iter().for_each(|(_,mut v)| {
            v.shift_ids(offset);
            self.points.insert(v.identifier, v);
        });
        other.connections.into_iter().for_each(|mut v| {
            v.shift_ids(offset);
            self.connections.insert(v);
        });
    }
}
