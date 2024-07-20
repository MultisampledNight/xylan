use ordered_float::OrderedFloat;

#[cfg(test)]
mod tests;

pub type Scalar = OrderedFloat<f32>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vector {
    x: Scalar,
    y: Scalar,
    z: Scalar,
}

impl<L, C> From<L> for Vector
where
    L: Into<[C; 3]>,
    C: Into<Scalar>,
{
    fn from(list: L) -> Self {
        let [x, y, z] = list.into();
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vertex(pub Vector);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Edge {
    a: Vertex,
    b: Vertex,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Plane {
    a: Edge,
    b: Edge,
}

trait Fill<O> {
    fn fill(self) -> O;
}

impl<C: Into<Scalar>> Fill<Vertex> for (C, C, C) {
    fn fill(self) -> Vertex {
        Vertex(self.into())
    }
}
