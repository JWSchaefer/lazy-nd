use lazy_nd::lazy_nd;
#[lazy_nd(dim = D)]
struct Test<const D: usize, const T: usize> {}
fn main() {}
