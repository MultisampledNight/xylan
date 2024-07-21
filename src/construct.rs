use ordered_float::OrderedFloat;
use ultraviolet::Vec3;

use crate::{Edge, Triangle, Vertex};
// TODO: there's no fundamental difference between NewX and From<...> for X lol

// HACK: for some reason, using AsPrimitive as generic doesn't lead to the resolution of
// .into() calls, only from() calls
// i think this is due to the double indirection?
// hence we just list here everything manually lmao

macro_rules! construct {
    ($(
        $postfix_trait:ident
        :: $postfix_method:ident
        => $target:ty
        : [$(
            $source:ty => |$args:pat_param| $op:expr
        ),* $(,)?]
    ),* $(,)?) => {$(
        $(
            impl From<$source> for $target {
                fn from(source: $source) -> Self {
                    (|$args: $source| $op)(source)
                }
            }
        )*

        pub trait $postfix_trait {
            fn $postfix_method(self) -> $target;
        }

        impl<T> $postfix_trait for T
        where
            T: Into<$target>,
        {
            fn $postfix_method(self) -> $target {
                self.into()
            }
        }
    )*}
}

construct! {
    NewVertex::vertex => Vertex : [
        [f32; 3] => |[x, y, z]| Self(Vec3::new(x, y, z)),
        [f32; 2] => |[x, y]| [x, y, 0.0].vertex(),
        [i32; 3] => |comp| comp.map(|c| c as f32).vertex(),
        [i32; 2] => |comp| comp.map(|c| c as f32).vertex(),

        (f32, f32, f32) => |(x, y, z)| [x, y, z].vertex(),
        (f32, f32) => |(x, y)| [x, y].vertex(),
        (i32, i32, i32) => |(x, y, z)| [x, y, z].vertex(),
        (i32, i32) => |(x, y)| [x, y].vertex(),
    ],
    NewEdge::edge => Edge : [
        [Vertex; 2] => |[a, b]| Self { a, b },
        (Vertex, Vertex) => |(a, b)| [a, b].edge(),
    ],
    NewTriangle::triangle => Triangle : [
        [Vertex; 3] => |vertices| Self::new(vertices),
        (Vertex, Vertex, Vertex) => |(a, b, c)| [a, b, c].triangle(),
        (Edge, Vertex) => |(edge, vertex)| [
            edge.a,
            edge.b,
            vertex,
        ].triangle(),
    ],
}

impl Triangle {
    /// Constructs a triangle while reordering the vertices to fit the struct's description.
    pub fn new(mut vertices: [Vertex; 3]) -> Self {
        let key = |vert: &Vertex| [
            vert.0.z.atan2(vert.0.y),
            vert.0.x.atan2(vert.0.y),
            // serves as a tie-breaker in case of same angle
            vert.0.mag_sq(),
        ].map(OrderedFloat);
        vertices.sort_unstable_by_key(key);

        let [a, b, c] = vertices;
        Self { a, b, c }
    }
}
