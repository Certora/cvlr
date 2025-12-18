//! Tests for cvlr_assert_that, cvlr_assert_all, cvlr_assume_that, cvlr_assume_all, and cvlr_rule_for_spec macros

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
    t.pass("tests/expand/test_cvlr_assert_that_true_and_if.rs");
    t.pass("tests/expand/test_cvlr_assert_all.rs");
    t.pass("tests/expand/test_cvlr_assume_that.rs");
    t.pass("tests/expand/test_cvlr_assume_that_true_and_if.rs");
    t.pass("tests/expand/test_cvlr_assume_all.rs");
    t.pass("tests/expand/test_cvlr_eval_that.rs");
    t.pass("tests/expand/test_cvlr_eval_all.rs");
    t.pass("tests/expand/test_cvlr_rule_for_spec.rs");
}
