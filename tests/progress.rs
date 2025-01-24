#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-minimum.rs");
    t.pass("tests/02-minimum_default.rs");
    t.pass("tests/03-minimum_multiple.rs");
    t.pass("tests/04-field.rs");
    t.compile_fail("tests/07-duplicate.rs");
}
