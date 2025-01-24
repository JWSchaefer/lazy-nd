use lazy_nd::lazy_nd;

#[lazy_nd(dim = G)]
struct Test<const D: usize> {}
fn main() {}
