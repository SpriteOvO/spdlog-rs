#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();

    t.compile_fail("tests/compile-fail/logging-macro-*.rs");
    t.compile_fail("tests/compile-fail/pattern-macro-*.rs");
    #[cfg(feature = "runtime-pattern")]
    t.compile_fail("tests/compile-fail/pattern-runtime-macro-*.rs");
    #[cfg(not(feature = "runtime-pattern"))]
    t.compile_fail("tests/compile-fail/pattern-runtime-disabled.rs");
}
