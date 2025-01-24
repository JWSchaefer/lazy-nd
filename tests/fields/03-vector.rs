use lazy_nd::lazy_nd;
#[lazy_nd(dim = D)]
struct Test<const D: usize> {
    #[vector(mass:f64)]
    #[scalar(position : f64)]
    field: f64,
}
fn main() {}
