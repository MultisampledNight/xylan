#[test]
fn plane() {
    let a = (0, 0, 0).fill();
    let b = (1, 0, 0).fill();

    let c = (a, b).edge();
    let plane = c.extrude(1, Along::Normal);
}
