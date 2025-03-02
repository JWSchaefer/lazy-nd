use lazy_nd::lazy_nd;

#[lazy_nd(dim = 2)]
struct Test {
    #[vector]
    position: f64,
}

fn main() {}
