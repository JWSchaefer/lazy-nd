#[test]
fn tests() {
    let t = trybuild::TestCases::new();

    // General
    t.pass("tests/general/01-minimum.rs");
    // Attrib
    t.compile_fail("tests/attrib/01-mismatch.rs");
    t.compile_fail("tests/attrib/02-missing.rs");
    t.compile_fail("tests/attrib/03-empty.rs");
    t.compile_fail("tests/attrib/04-wrong.rs");
    t.compile_fail("tests/attrib/04-wrong.rs");
    t.compile_fail("tests/attrib/05-extra.rs");
    // Generics
    t.pass("tests/generics/01-minimum_default.rs");
    t.pass("tests/generics/02-minimum_multiple.rs");
    t.compile_fail("tests/generics/03-duplicate.rs");
    t.compile_fail("tests/generics/04-mismatch.rs");
    t.compile_fail("tests/generics/05-match_mismatch.rs");
    t.compile_fail("tests/generics/06-type.rs");
    // Fields
    t.pass("tests/fields/01-field.rs");
    t.pass("tests/fields/02-scalar.rs");
    t.pass("tests/fields/03-vector.rs");
    t.compile_fail("tests/fields/04-duplicate_nd.rs");
    t.compile_fail("tests/fields/05-duplicate_field.rs");
    t.compile_fail("tests/fields/06-duplicate_cross.rs");
}
