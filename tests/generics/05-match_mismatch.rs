use lazy_nd::lazy_nd;
#[lazy_nd(dim = D)]
struct Test<const T: usize, D> {}
fn main() {}
