use lazy_nd::lazy_nd;

#[lazy_nd(dim = D, extra = 1)]
struct Test<const D: usize> {}
fn main() {}
