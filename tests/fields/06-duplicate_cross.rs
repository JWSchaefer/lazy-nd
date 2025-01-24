use lazy_nd::lazy_nd;
#[lazy_nd(dim = D)]
struct Test<const D: usize> {
    #[scalar(mass:f64)]
    #[scalar(position : f64)]
    mass: f64,
}
fn main() {}
