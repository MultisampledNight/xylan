use crate::{Edge, Fill, Plane, Vertex};

#[test]
fn plane() {
    let a = (0, 0, 0).fill();
    let b = (1, 0, 0).fill();

    let c = (a, b).fill();
    let plane = c.extrude(1, Along::Normal);

    assert_eq!(
        plane,
        Plane {
            a: Edge {
                a: Vertex([0, 0, 0].into()),
                b: Vertex([1, 0, 0].into()),
            },
            b: Edge {
                a: Vertex([0, 1, 0].into()),
                b: Vertex([1, 1, 0].into()),
            },
        }
    )
}
