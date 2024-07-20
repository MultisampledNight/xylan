use ultraviolet::Vec3;

pub struct Context;

impl Context {
    pub fn fill<A, B, O>(a: A, b: B) -> O
    where
        (A, B): Fill<O>
    {
        (a, b).fill()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex(pub Vec3);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Edge {
    pub a: Vertex,
    pub b: Vertex,
}

// TODO: need to account for ccw and cw somehow? maybe just store tris?
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {
    pub a: Edge,
    pub b: Edge,
}

pub trait Fill<O> {
    fn fill(self) -> O;
}

impl Fill<Edge> for (Vertex, Vertex) {
    fn fill(self) -> Edge {
        Edge { a: self.0, b: self.1 }
    }
}

