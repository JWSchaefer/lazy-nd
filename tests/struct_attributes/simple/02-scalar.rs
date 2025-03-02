use lazy_nd::lazy_nd;

#[lazy_nd]
struct Test {
    #[scalar]
    id: u64,
}

fn main() {}
