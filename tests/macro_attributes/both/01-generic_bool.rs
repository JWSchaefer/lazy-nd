use lazy_nd::lazy_nd;

#[lazy_nd(dim = D, inner = true)]
struct Test<const D: usize> {}

fn main() {}
