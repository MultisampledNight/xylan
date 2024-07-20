#[cfg(test)]
mod tests;

pub mod construct;
pub mod prelude;

use ultraviolet::Vec3;

pub struct Context;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex(pub Vec3);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Edge {
    pub a: Vertex,
    pub b: Vertex,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    pub a: Vertex,
    pub b: Vertex,
    pub c: Vertex,
}

// TODO: need to account for ccw and cw somehow? maybe just store tris?
#[derive(Clone, Debug, PartialEq)]
pub struct Face {
    pub tris: Vec<Triangle>,
}
