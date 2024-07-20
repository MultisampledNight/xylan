use ultraviolet::Vec3;

use crate::{Edge, Vertex};
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
            $source:ty => $op:expr
        ),* $(,)?]
    ),* $(,)?) => {$(
        $(
            impl From<$source> for $target {
                fn from(source: $source) -> Self {
                    ($op)(source)
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
        (f32, f32, f32) => |(x, y, z)| Self(Vec3::new(x, y, z)),
        (f32, f32) => |(x, y)| (x, y, 0.0).vertex(),
        (i32, i32, i32) => |(x, y, z)| (x as f32, y as f32, z as f32).vertex(),
        (i32, i32) => |(x, y)| (x as f32, y as f32, 0.0).vertex(),
    ],
    NewEdge::edge => Edge : [
        (Vertex, Vertex) => |(a, b)| Self { a, b },
    ],
}
