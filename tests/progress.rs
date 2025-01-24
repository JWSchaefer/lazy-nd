#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    // Attrib
    t.pass("tests/attrib/01-minimum.rs");
    t.compile_fail("tests/attrib/02-mismatch.rs");
    t.compile_fail("tests/attrib/03-missing.rs");
    t.compile_fail("tests/attrib/04-empty.rs");

    // Fields
}
