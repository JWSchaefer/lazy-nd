use lazy_nd::lazy_nd;
#[lazy_nd(dim = D)]
struct Test<const D: usize> {
    #[vector]
    position: f64,
    #[vector(3)]
    velocity: f64,
    #[vector(D)]
    acceleration: f64,
    #[scalar]
    mass: f64,
    id: uint,
    name: &str,
}
fn main() {}
