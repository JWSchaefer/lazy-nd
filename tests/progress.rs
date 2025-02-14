#[test]
fn tests() {
    let t = trybuild::TestCases::new();

    // General
    t.pass("tests/general/01-minimum.rs");
    // Struct Attributes - Dim
    t.pass("tests/struct_attributes/dim/01-usize.rs");
    t.pass("tests/struct_attributes/dim/02-generic.rs");
    t.compile_fail("tests/struct_attributes/dim/03-invalid.rs");
    t.compile_fail("tests/struct_attributes/dim/04-undefined.rs");
    t.compile_fail("tests/struct_attributes/dim/05-incomplete.rs");
    t.compile_fail("tests/struct_attributes/dim/06-incomplete_eq.rs");
    t.compile_fail("tests/struct_attributes/dim/07-incomplete_eq_comma.rs");
    // Struct Attributes - Inner
    t.pass("tests/struct_attributes/inner/01-bool.rs");
    t.compile_fail("tests/struct_attributes/inner/02-invalid.rs");
    t.compile_fail("tests/struct_attributes/inner/03-incomplete.rs");
    t.compile_fail("tests/struct_attributes/inner/04-incomplete_eq.rs");
    t.compile_fail("tests/struct_attributes/inner/03-incomplete_eq_comma.rs");
}
