#[test]
fn tests() {
    let t = trybuild::TestCases::new();

    // General
    t.pass("tests/general/01-minimum.rs");
    // Struct Attributes
    t.pass("tests/struct_attributes/01-dim_usize.rs");
    t.pass("tests/struct_attributes/02-dim_generic.rs");
    t.compile_fail("tests/struct_attributes/03-dim_generic_invalid.rs");
    t.compile_fail("tests/struct_attributes/04-dim_generic_undefined.rs");
    t.pass("tests/struct_attributes/05-inner_true.rs");
    t.pass("tests/struct_attributes/06-inner_false.rs");
    t.compile_fail("tests/struct_attributes/07-inner_invalid.rs");
    t.pass("tests/struct_attributes/08-both_usize_bool.rs");
    t.pass("tests/struct_attributes/09-both_generic_bool.rs");
    t.compile_fail("tests/struct_attributes/10-both_invalid_bool.rs");
    t.compile_fail("tests/struct_attributes/11-both_undefined_bool.rs");
    t.compile_fail("tests/struct_attributes/12-both_usize_invalid.rs");
    t.compile_fail("tests/struct_attributes/13-both_generic_invalid.rs");
}
