#[allow(unused)]
use indoc::indoc;

use crate::{prelude::*, Edge, Vertex};

#[test]
fn edge() {
    let edge = (Vertex::from((0, 0)), (1, 0).vertex()).edge();
    assert_eq!(
        edge,
        Edge {
            a: Vertex((0.0, 0.0, 0.0).into()),
            b: Vertex((1.0, 0.0, 0.0).into()),
        }
    );
}

/*
#[test]
fn future() {
    let mut ctx = Context::new();

    ((0, 0, 0).vertex(), (1, 0, 0).vertex())
        .edge()
        .extrude(Along::Axis(Y), 1)
        .store(&mut ctx);

    assert_eq!(
        ctx.into_obj(),
        indoc! {"
            v 0.0 0.0 0.0
            v 1.0 0.0 0.0
            v 1.0 1.0 0.0
            v 0.0 1.0 0.0
            f 1 2 3 4
        "},
    );
}
*/
