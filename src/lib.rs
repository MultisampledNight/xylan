//! # Assumptions
//!
//! The library is thought to work like Blender,
//! but used programmatically from Rust.
//! Hence, the assumptions are similar to Blender:
//!
//! - Triangles store their vertices in counter-clockwise order.
//! - For checking counter-clockwiseness,
//!     the coordinate system is *left-handed*:
//!     - `x+` to the camera
//!     - `y+` right-hand
//!     - `z+` up

#[cfg(test)]
mod tests;

pub mod construct;
pub mod ops;
pub mod prelude;

use std::iter;

use construct::NewEdge;
use ultraviolet::Vec3;

pub struct Context;

/// Point somewhere in space.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct Vertex(pub Vec3);

/// Connection between 2 points.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Edge {
    pub a: Vertex,
    pub b: Vertex,
}

/// Spans some surface between 3 points.
///
/// Guranteed to store its vertices in counter-clockwise order.
/// 2 triangles are the same if they contain the same points,
/// regardless of construction order.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    a: Vertex,
    b: Vertex,
    c: Vertex,
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
    /// Note: Vertices are guaranteed to be returned in counter-clockwise order.
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
