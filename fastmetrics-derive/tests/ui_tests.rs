#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    // Tests that should compile successfully
    t.pass("tests/ui/pass/**/*.rs");
    // Tests that should fail to compile with specific errors
    t.compile_fail("tests/ui/fail/**/*.rs");
}
