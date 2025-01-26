// TODO: Fix this error
use lazy_nd::lazy_nd;
#[lazy_nd(dim = D)]
struct Test<const D: bool> {}
fn main() {}
