#[cfg(test)]
mod tests;

pub mod construct;
pub mod ops;
pub mod prelude;

use std::iter;

use construct::NewEdge;
use ultraviolet::Vec3;

pub struct Context;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex(pub Vec3);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Edge {
    pub a: Vertex,
    pub b: Vertex,
}

// TODO: enforce ccw
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    pub a: Vertex,
    pub b: Vertex,
    pub c: Vertex,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StlMesh {
    pub tris: Vec<Triangle>,
}

pub trait Meshish: Sized {
    fn vertices(self) -> impl IntoIterator<Item = Vertex>;

    fn edges(self) -> impl IntoIterator<Item = Edge> {
        iter::empty()
    }

    fn tris(self) -> impl IntoIterator<Item = Triangle> {
        iter::empty()
    }
}

impl Meshish for Vertex {
    fn vertices(self) -> impl IntoIterator<Item = Self> {
        iter::once(self)
    }
}

impl Meshish for Edge {
    fn vertices(self) -> impl IntoIterator<Item = Vertex> {
        vec![self.a, self.b]
    }

    fn edges(self) -> impl IntoIterator<Item = Self> {
        iter::once(self)
    }
}

impl Meshish for Triangle {
    fn vertices(self) -> impl IntoIterator<Item = Vertex> {
        vec![self.a, self.b, self.c]
    }

    fn edges(self) -> impl IntoIterator<Item = Edge> {
        vec![
            (self.a, self.b).edge(),
            (self.b, self.c).edge(),
            (self.c, self.a).edge(),
        ]
    }

    fn tris(self) -> impl IntoIterator<Item = Self> {
        iter::once(self)
    }
}

impl Meshish for StlMesh {
    fn vertices(self) -> impl IntoIterator<Item = Vertex> {
        self.tris.into_iter().flat_map(|tri| tri.vertices())
    }

    fn edges(self) -> impl IntoIterator<Item = Edge> {
        self.tris.into_iter().flat_map(|tri| tri.edges())
    }

    fn tris(self) -> impl IntoIterator<Item = Triangle> {
        self.tris
    }
}
