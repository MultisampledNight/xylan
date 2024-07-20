use ultraviolet::Vec3;

use crate::{Along, Edge, Extrude, Fill, Plane, Vertex};

#[test]
fn plane() {
    let a = (0.0, 0.0, 0.0).fill();
    let b = (1.0, 0.0, 0.0).fill();

    let c = (a, b).fill();
    let plane = c.extrude(1.0, Along::Normal);

    assert_eq!(
        plane,
        Plane {
            a: Edge {
                a: Vertex(Vec3::new(0.0, 0.0, 0.0)),
                b: Vertex(Vec3::new(1.0, 0.0, 0.0)),
            },
            b: Edge {
                a: Vertex(Vec3::new(0.0, 1.0, 0.0)),
                b: Vertex(Vec3::new(1.0, 1.0, 0.0)),
            },
        }
    )
}
