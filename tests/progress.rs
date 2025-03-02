#[test]
fn tests() {
    let t = trybuild::TestCases::new();

    // General
    t.pass("tests/general/01-minimum.rs");
    // Macro Attributes - Dim
    t.pass("tests/macro_attributes/dim/01-usize.rs");
    t.pass("tests/macro_attributes/dim/02-generic.rs");
    t.compile_fail("tests/macro_attributes/dim/03-invalid.rs");
    t.compile_fail("tests/macro_attributes/dim/04-undefined.rs");
    t.compile_fail("tests/macro_attributes/dim/05-incomplete.rs");
    t.compile_fail("tests/macro_attributes/dim/06-incomplete_eq.rs");
    t.compile_fail("tests/macro_attributes/dim/07-incomplete_eq_comma.rs");
    t.compile_fail("tests/macro_attributes/dim/08-overdefined.rs");
    // Macro Attributes - Inner
    t.pass("tests/macro_attributes/inner/01-bool.rs");
    t.compile_fail("tests/macro_attributes/inner/02-invalid.rs");
    t.compile_fail("tests/macro_attributes/inner/03-incomplete.rs");
    t.compile_fail("tests/macro_attributes/inner/04-incomplete_eq.rs");
    t.compile_fail("tests/macro_attributes/inner/05-incomplete_eq_comma.rs");
    t.compile_fail("tests/macro_attributes/inner/06-overdefined.rs");
    // Struct Attributes
    t.pass("tests/struct_attributes/simple/01-fields.rs");
    t.pass("tests/struct_attributes/simple/02-scalar.rs");
    t.pass("tests/struct_attributes/simple/03-vector_underdefined_valid.rs");
    t.pass("tests/struct_attributes/simple/04-vector_literal.rs");
    t.pass("tests/struct_attributes/simple/05-vector_generic.rs");
    t.compile_fail("tests/struct_attributes/simple/06-vector_invalid_bool.rs");
    t.compile_fail("tests/struct_attributes/simple/07-vector_invalid_float.rs");
    t.compile_fail(
        "tests/struct_attributes/simple/08-vector_invalid_string.rs",
    );
}
