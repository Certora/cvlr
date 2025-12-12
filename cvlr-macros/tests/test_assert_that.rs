//! Tests for cvlr_assert_that, cvlr_assert_all, cvlr_assume_that, and cvlr_assume_all macros

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
    t.pass("tests/expand/test_cvlr_assert_all.rs");
    t.pass("tests/expand/test_cvlr_assume_that.rs");
    t.pass("tests/expand/test_cvlr_assume_all.rs");
    t.pass("tests/expand/test_cvlr_eval_that.rs");
    t.pass("tests/expand/test_cvlr_eval_all.rs");
}
