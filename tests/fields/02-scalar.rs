use lazy_nd::lazy_nd;
#[lazy_nd(dim = D)]
struct Test<const D: usize> {
    #[scalar(mass:f64)]
    field: f64,
}
fn main() {}
