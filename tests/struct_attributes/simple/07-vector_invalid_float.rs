use lazy_nd::lazy_nd;

#[lazy_nd]
struct Test {
    #[vector(3.2)]
    position: f64,
}

fn main() {}
