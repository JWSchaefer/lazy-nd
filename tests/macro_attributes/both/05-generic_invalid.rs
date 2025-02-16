use lazy_nd::lazy_nd;

#[lazy_nd(dim = D, inner = 10)]
struct Test<const D: usize> {}

fn main() {}
