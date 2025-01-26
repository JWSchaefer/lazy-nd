use lazy_nd::lazy_nd;
#[lazy_nd(dim = D)]
struct Test<const D: usize = 2> {}
fn main() {}
