#[test]
#[cfg_attr(miri, ignore)] // Process spawning is not supported in Miri emulation
fn compile_fail() {
    let t = trybuild::TestCases::new();

    t.compile_fail("tests/compile_fail/logging_macro_*.rs");
    t.compile_fail("tests/compile_fail/pattern_macro_*.rs");
    #[cfg(feature = "runtime-pattern")]
    t.compile_fail("tests/compile_fail/pattern_runtime_macro_*.rs");
    #[cfg(not(feature = "runtime-pattern"))]
    t.compile_fail("tests/compile_fail/pattern_runtime_disabled.rs");
}
