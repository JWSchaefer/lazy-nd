use lazy_nd::lazy_nd;

#[lazy_nd(lim = D)]
struct Test<const D: usize> {}
fn main() {}
