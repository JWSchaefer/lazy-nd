use lazy_nd::lazy_nd;

#[lazy_nd]
struct Test {
    #[vector(false)]
    position: f64,
}

fn main() {}
