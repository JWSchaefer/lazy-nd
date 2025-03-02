use lazy_nd::lazy_nd;

#[lazy_nd]
struct Test {
    #[vector("dim")]
    position: f64,
}

fn main() {}
