use ultraviolet::Vec3;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex(pub Vec3);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Edge {
    a: Vertex,
    b: Vertex,
}

// TODO: need to account for ccw and cw somehow? maybe just store tris?
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {
    a: Edge,
    b: Edge,
}

pub trait Fill<O> {
    fn fill(self) -> O;
}

impl<C: Into<f32>> Fill<Vertex> for (C, C, C) {
    fn fill(self) -> Vertex {
        let array: [C; 3] = self.into();
        let concrete = array.map(Into::into);
        Vertex(concrete.into())
    }
}

impl Fill<Edge> for (Vertex, Vertex) {
    fn fill(self) -> Edge {
        let (a, b) = self;
        Edge { a, b }
    }
}

pub trait Extrude<O> {
    fn extrude<C: Into<f32>>(self, amount: C, along: Along) -> O;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Along {
    // TODO: on edges, does that refer to the left-hand side or the right-hand side?
    // kinda thinking we need an individual Along specifier for edges vs. faces...
    Normal,
}

impl Extrude<Plane> for Edge {
    fn extrude<C: Into<f32>>(self, amount: C, _along: Along) -> Plane {
        // just rotate a → b vector by 90 deg counterclockwise
        // then set length to amount
        // add it to a + b
        // boom
        let a_to_b = self.b.0 - self.a.0;

        todo!()
    }
}
