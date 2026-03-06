use std::{collections::{HashMap, HashSet}, marker::PhantomData};

use rerun::external::glam::usize;



/// represent a unique identifier of one contact point that the
/// algorithm decided to place
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub  struct ContactIdentifier(usize);

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
}

/// represent the position of one contact point
#[derive(Debug, Clone)]
pub struct ContactPoint {
    pub identifier: ContactIdentifier,
    pub x: f32,
    pub y: f32
}

/// uniquely identify one support structure that can be mutated, evaluated ecc.
#[derive(Debug, Clone, Default)]
pub struct ContactPointsGene {
    points: HashMap<ContactIdentifier, ContactPoint>,
    connections: HashSet<Connection>
}
