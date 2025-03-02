use lazy_nd::lazy_nd;

#[lazy_nd(dim = D)]
struct Test<const D: usize> {
    #[vector]
    position: f64,
}

fn main() {}
