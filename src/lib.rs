use ultraviolet::{Rotor3, Vec3};

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
    // TODO: on edges, to which one of the infinite amount of normals does this refer to?
    Normal,
}

impl Extrude<Plane> for Edge {
    fn extrude<C: Into<f32>>(self, amount: C, _along: Along) -> Plane {
        // TODO: this doesn't make sense from any perspective, it is far too naive
        // just rotate a → b vector by 90 deg counterclockwise (assuming on the z plane for now)
        // then set length to amount
        // add it to a + b
        // boom
        let a_to_b = self.b.0 - self.a.0;
        let rotation = Rotor3::from_rotation_between(Vec3::unit_x(), Vec3::unit_y());
        let mut normal = rotation * a_to_b;
        normal.normalize();

        let extrusion = normal * amount.into();

        let opposite = Self {
            a: Vertex(self.a.0 + extrusion),
            b: Vertex(self.b.0 + extrusion),
        };

        Plane {
            a: self,
            b: opposite,
        }
    }
}
