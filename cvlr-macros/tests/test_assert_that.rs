//! Tests for cvlr_assert_that macro

#[test]
fn test_cvlr_assert_that_macro_expansion() {
    macrotest::expand("tests/expand/*.rs");
}

#[test]
fn test_cvlr_assert_that_compiles() {
    let t = trybuild::TestCases::new();
    t.pass("tests/expand/test_cvlr_assert_that_comparisons.rs");
    t.pass("tests/expand/test_cvlr_assert_that_guarded_comparisons.rs");
    t.pass("tests/expand/test_cvlr_assert_that_booleans.rs");
}
