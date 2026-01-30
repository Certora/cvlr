//! Tests that validate macro expansion for cvlr-spec macros

#[test]
fn test_cvlr_predicate_macro_expansion() {
    macrotest::expand("tests/expand/*.rs");
}
