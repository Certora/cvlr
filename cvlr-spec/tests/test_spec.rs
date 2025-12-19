//! Tests for cvlr-spec crate

extern crate cvlr;

use cvlr_spec::spec::CvlrLemma;
use cvlr_spec::*;

// Test context type
#[derive(Clone, Copy, Debug, PartialEq, Eq, cvlr::derive::Nondet, cvlr::derive::CvlrLog)]
struct TestCtx {
    x: i32,
    y: i32,
    flag: bool,
}

// Test boolean expression that checks if x > 0
#[derive(Copy, Clone)]
struct XPositive;

// Test boolean expression that checks if y > 0
#[derive(Copy, Clone)]
struct YPositive;

// Boolean expression for (TestCtx, TestCtx) tuple that checks if post.y > 0
#[derive(Copy, Clone)]
struct PostYPositive;

impl CvlrFormula for XPositive {
    type Context = TestCtx;
    fn eval(&self, ctx: &Self::Context) -> bool {
        ctx.x > 0
    }
}

impl CvlrFormula for YPositive {
    type Context = TestCtx;
    fn eval(&self, ctx: &Self::Context) -> bool {
        ctx.y > 0
    }
}

impl CvlrFormula for PostYPositive {
    type Context = TestCtx;
    fn eval(&self, ctx: &Self::Context) -> bool {
        ctx.y > 0
    }
}

#[test]
fn test_cvlr_true() {
    let ctx = TestCtx {
        x: 0,
        y: 0,
        flag: false,
    };
    let true_expr = cvlr_true::<TestCtx>();
    assert!(true_expr.eval(&ctx));

    let ctx2 = TestCtx {
        x: 42,
        y: -1,
        flag: true,
    };
    assert!(true_expr.eval(&ctx2));
}

#[test]
fn test_cvlr_and() {
    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    let and_expr = cvlr_and(XPositive, YPositive);
    assert!(and_expr.eval(&ctx));

    let ctx2 = TestCtx {
        x: -1,
        y: 10,
        flag: true,
    };
    assert!(!and_expr.eval(&ctx2));

    let ctx3 = TestCtx {
        x: 5,
        y: -1,
        flag: true,
    };
    assert!(!and_expr.eval(&ctx3));

    let ctx4 = TestCtx {
        x: -1,
        y: -1,
        flag: true,
    };
    assert!(!and_expr.eval(&ctx4));
}

#[test]
fn test_cvlr_and_chained() {
    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    let true_expr = cvlr_true::<TestCtx>();
    let and1 = cvlr_and(XPositive, YPositive);
    let and2 = cvlr_and(and1, true_expr);
    assert!(and2.eval(&ctx));
}

#[test]
fn test_cvlr_implies() {
    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    // x > 0 -> y > 0 (both true, so true)
    let impl_expr = cvlr_implies(XPositive, YPositive);
    assert!(impl_expr.eval(&ctx));

    // x > 0 -> y > 0 (antecedent true, consequent false, so false)
    let ctx2 = TestCtx {
        x: 5,
        y: -1,
        flag: true,
    };
    assert!(!impl_expr.eval(&ctx2));

    // x > 0 -> y > 0 (antecedent false, so true regardless of consequent)
    let ctx3 = TestCtx {
        x: -1,
        y: -1,
        flag: true,
    };
    assert!(impl_expr.eval(&ctx3));

    // x > 0 -> y > 0 (antecedent false, consequent true, so true)
    let ctx4 = TestCtx {
        x: -1,
        y: 10,
        flag: true,
    };
    assert!(impl_expr.eval(&ctx4));
}

#[test]
fn test_state_pair_new() {
    let ctx1 = TestCtx {
        x: 1,
        y: 2,
        flag: true,
    };
    let ctx2 = TestCtx {
        x: 3,
        y: 4,
        flag: false,
    };
    let pair = (&ctx1, &ctx2);

    assert_eq!(pair.0, &ctx1);
    assert_eq!(pair.1, &ctx2);
    // For tuple, .0 is post, .1 is pre
    assert_eq!(pair.1, &ctx2);
    assert_eq!(pair.0, &ctx1);
}

#[test]
fn test_state_pair_singleton() {
    let ctx = TestCtx {
        x: 1,
        y: 2,
        flag: true,
    };
    let pair = (&ctx, &ctx);

    assert_eq!(pair.0, &ctx);
    assert_eq!(pair.1, &ctx);
    assert_eq!(pair.1, &ctx);
    assert_eq!(pair.0, &ctx);
}

#[test]
fn test_state_pair_deref() {
    let ctx = TestCtx {
        x: 1,
        y: 2,
        flag: true,
    };
    let pair = (&ctx, &ctx);

    // Test tuple access
    assert_eq!(pair.0.x, 1);
    assert_eq!(pair.0.y, 2);
    assert_eq!(pair.0.flag, true);
}

#[test]
fn test_cvlr_def_predicate() {
    cvlr_def_predicate! {
        pred XGreaterThanZero(c: TestCtx) {
            c.x > 0
        }
    }

    let ctx1 = TestCtx {
        x: 5,
        y: 0,
        flag: false,
    };
    let ctx2 = TestCtx {
        x: -1,
        y: 0,
        flag: false,
    };

    let pred = XGreaterThanZero;
    assert!(pred.eval(&ctx1));
    assert!(!pred.eval(&ctx2));
}

#[test]
fn test_cvlr_def_predicate_multiple_conditions() {
    cvlr_def_predicate! {
        pred XAndYPositive(c: TestCtx) {
            c.x > 0;
            c.y > 0
        }
    }

    let ctx1 = TestCtx {
        x: 5,
        y: 10,
        flag: false,
    };
    let ctx2 = TestCtx {
        x: -1,
        y: 10,
        flag: false,
    };
    let ctx3 = TestCtx {
        x: 5,
        y: -1,
        flag: false,
    };

    let pred = XAndYPositive;
    assert!(pred.eval(&ctx1));
    assert!(!pred.eval(&ctx2));
    assert!(!pred.eval(&ctx3));
}

cvlr_def_states_predicate! {
    pred XIncreased([ c, o ] : TestCtx) {
        c.x > o.x
    }
}

#[test]
fn test_cvlr_def_two_predicate() {
    let pre = TestCtx {
        x: 1,
        y: 0,
        flag: false,
    };
    let post = TestCtx {
        x: 5,
        y: 0,
        flag: false,
    };

    let pred = XIncreased;
    assert!(pred.eval_with_states(&post, &pre));

    assert!(!pred.eval_with_states(&pre, &post));
}

cvlr_def_states_predicate! {
    pred XAndYIncreased([ c, o ] : TestCtx) {
        c.x > o.x;
        c.y > o.y
    }
}

#[test]
fn test_cvlr_def_two_predicate_multiple_conditions() {
    let pre = TestCtx {
        x: 1,
        y: 2,
        flag: false,
    };
    let post = TestCtx {
        x: 5,
        y: 10,
        flag: false,
    };

    let pred = XAndYIncreased;
    assert!(pred.eval_with_states(&post, &pre));

    let post2 = TestCtx {
        x: 5,
        y: 1,
        flag: false,
    };
    assert!(!pred.eval_with_states(&post2, &pre));
}

#[test]
fn test_cvlr_spec() {
    // Create a spec: requires x > 0, ensures y > 0
    let requires = XPositive;
    let ensures = PostYPositive;

    let spec = cvlr_spec(requires, ensures);

    // Test assume_requires
    let pre = TestCtx {
        x: 5,
        y: 0,
        flag: false,
    };
    spec.assume_requires(&pre);

    // Test check_ensures
    let post = TestCtx {
        x: 5,
        y: 10,
        flag: false,
    };
    spec.check_ensures(&post, &pre);
}
// Define predicates for the ensures condition
cvlr_def_states_predicate! {
    pred PostXPositive([ c, o ] : TestCtx) {
        c.x > 0
    }
}

#[test]
fn test_cvlr_spec_with_implication() {
    cvlr_def_states_predicate! {
        pred PostYPositive([ c, o ] : TestCtx) {
            c.y > 0
        }
    }

    // Create a spec: requires x > 0, ensures if x > 0 then y > 0
    // Test that cvlr_implies preserves HRTB bounds
    let requires = XPositive;
    let ensures = cvlr_implies(PostXPositive, PostYPositive);

    let spec = cvlr_spec(requires, ensures);

    let pre = TestCtx {
        x: 5,
        y: 0,
        flag: false,
    };
    spec.assume_requires(&pre);

    // This should pass because the ensures is an implication
    // and the antecedent might be false in post state
    let post = TestCtx {
        x: -1,
        y: 0,
        flag: false,
    };
    spec.check_ensures(&post, &pre);
}

#[test]
fn test_cvlr_spec_with_and() {
    // Test that cvlr_and preserves HRTB bounds
    let requires = XPositive;
    let ensures = cvlr_and(PostXPositive, PostYPositive);

    let spec = cvlr_spec(requires, ensures);

    let pre = TestCtx {
        x: 5,
        y: 0,
        flag: false,
    };
    spec.assume_requires(&pre);

    let post = TestCtx {
        x: 5,
        y: 10,
        flag: false,
    };
    spec.check_ensures(&post, &pre);
}

#[test]
fn test_cvlr_invar_spec() {
    // Create an invariant spec: assumption x > 0, invariant y > 0
    let assumption = XPositive;
    let invariant = YPositive;

    let spec = cvlr_invar_spec(assumption, invariant);

    // Test assume_requires - should assume both
    let pre = TestCtx {
        x: 5,
        y: 10,
        flag: false,
    };
    spec.assume_requires(&pre);

    // Test check_ensures - should assert invariant on post state
    let post = TestCtx {
        x: 5,
        y: 10,
        flag: false,
    };
    spec.check_ensures(&post, &pre);
}

#[test]
fn test_cvlr_invar_spec_accessors() {
    let assumption = XPositive;
    let invariant = YPositive;

    // Now cvlr_invar_spec returns the concrete type, so we can use accessors
    let spec = cvlr_invar_spec(assumption, invariant);

    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: false,
    };
    assert!(spec.assumption().eval(&ctx));
    assert!(spec.invariant().eval(&ctx));

    let ctx2 = TestCtx {
        x: -1,
        y: 10,
        flag: false,
    };
    assert!(!spec.assumption().eval(&ctx2));
    assert!(spec.invariant().eval(&ctx2));
}

#[test]
fn test_cvlr_bool_expr_assert() {
    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    let expr = cvlr_and(XPositive, YPositive);

    // This should not panic since both are true
    expr.assert(&ctx);
}

#[test]
fn test_cvlr_bool_expr_assume() {
    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    let expr = cvlr_and(XPositive, YPositive);

    // This should not panic since both are true
    expr.assume(&ctx);
}

#[test]
fn test_cvlr_true_optimized() {
    let ctx = TestCtx {
        x: 0,
        y: 0,
        flag: false,
    };
    let true_expr = cvlr_true::<TestCtx>();

    // CvlrTrue has optimized assert and assume that do nothing
    true_expr.assert(&ctx);
    true_expr.assume(&ctx);
}

#[test]
fn test_nested_expressions() {
    // Test complex nested expressions
    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };

    // (x > 0 && y > 0) && true
    let expr1 = cvlr_and(cvlr_and(XPositive, YPositive), cvlr_true::<TestCtx>());
    assert!(expr1.eval(&ctx));

    // (x > 0 -> y > 0) && (y > 0 -> x > 0)
    let expr2 = cvlr_and(
        cvlr_implies(XPositive, YPositive),
        cvlr_implies(YPositive, XPositive),
    );
    assert!(expr2.eval(&ctx));
}

#[test]
fn test_state_pair_lifetime() {
    let ctx1 = TestCtx {
        x: 1,
        y: 2,
        flag: true,
    };
    let ctx2 = TestCtx {
        x: 3,
        y: 4,
        flag: false,
    };

    {
        let pair = (ctx1, ctx2);
        assert_eq!(pair.0.x, 1);
        assert_eq!(pair.1.x, 3);
    }

    // pair is dropped, but contexts are still valid
    assert_eq!(ctx1.x, 1);
    assert_eq!(ctx2.x, 3);
}

#[test]
fn test_cvlr_def_predicates() {
    cvlr_def_predicates! {
        pred XIsPositive(c: TestCtx) {
            c.x > 0
        }
        pred YIsPositive(c: TestCtx) {
            c.y > 0
        }
        pred FlagIsTrue(c: TestCtx) {
            c.flag
        }
    }

    let ctx1 = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    let ctx2 = TestCtx {
        x: -1,
        y: 10,
        flag: false,
    };

    assert!(XIsPositive.eval(&ctx1));
    assert!(!XIsPositive.eval(&ctx2));
    assert!(YIsPositive.eval(&ctx1));
    assert!(YIsPositive.eval(&ctx2));
    assert!(FlagIsTrue.eval(&ctx1));
    assert!(!FlagIsTrue.eval(&ctx2));
}

#[test]
fn test_cvlr_def_states_predicates() {
    cvlr_def_states_predicates! {
        pred XIncreased([ c, o ] : TestCtx) {
            c.x > o.x
        }
        pred YIncreased([ c, o ] : TestCtx) {
            c.y > o.y
        }
        pred BothIncreased([ c, o ] : TestCtx) {
            c.x > o.x;
            c.y > o.y
        }
    }

    let pre = TestCtx {
        x: 1,
        y: 2,
        flag: false,
    };
    let post = TestCtx {
        x: 5,
        y: 10,
        flag: false,
    };

    assert!(XIncreased.eval_with_states(&post, &pre));
    assert!(YIncreased.eval_with_states(&post, &pre));
    assert!(BothIncreased.eval_with_states(&post, &pre));

    let post2 = TestCtx {
        x: 5,
        y: 1,
        flag: false,
    };
    assert!(XIncreased.eval_with_states(&post2, &pre));
    assert!(!YIncreased.eval_with_states(&post2, &pre));
    assert!(!BothIncreased.eval_with_states(&post2, &pre));
}

#[test]
fn test_cvlr_predicate() {
    // Test cvlr_predicate! macro creates an anonymous predicate
    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };

    let pred = cvlr_predicate! { | c : TestCtx | -> {
        c.x > 0;
        c.y > 0
    } };

    assert!(pred.eval(&ctx));

    let ctx2 = TestCtx {
        x: -1,
        y: 10,
        flag: true,
    };
    assert!(!pred.eval(&ctx2));
}

#[test]
fn test_cvlr_predicate_single_condition() {
    let ctx = TestCtx {
        x: 5,
        y: 0,
        flag: false,
    };

    let pred = cvlr_predicate! { | c : TestCtx | -> {
        c.x > 0
    } };

    assert!(pred.eval(&ctx));

    let ctx2 = TestCtx {
        x: -1,
        y: 0,
        flag: false,
    };
    assert!(!pred.eval(&ctx2));
}

#[test]
fn test_cvlr_lemma() {
    // Test cvlr_lemma! macro creates a lemma
    cvlr_lemma! {
        TestLemma(c: TestCtx) {
            requires -> {
                c.x > 0
            }
            ensures -> {
                c.x > 0;
                c.y >= 0
            }
        }
    }

    let lemma = TestLemma;

    // Test requires() returns a predicate
    let ctx1 = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    assert!(lemma.requires().eval(&ctx1));

    let ctx2 = TestCtx {
        x: -1,
        y: 10,
        flag: true,
    };
    assert!(!lemma.requires().eval(&ctx2));

    // Test ensures() returns a predicate
    assert!(lemma.ensures().eval(&ctx1));

    let ctx3 = TestCtx {
        x: 5,
        y: -1,
        flag: true,
    };
    assert!(!lemma.ensures().eval(&ctx3));
}

#[test]
fn test_cvlr_lemma_verify_with_context() {
    cvlr_lemma! {
        PositiveXLemma(c: TestCtx) {
            requires -> {
                c.x > 0
            }
            ensures -> {
                c.x > 0
            }
        }
    }

    let lemma = PositiveXLemma;

    // Test verify_with_context - should assume requires and assert ensures
    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };

    // This should not panic since requires and ensures both hold
    lemma.verify_with_context(&ctx);
}

#[test]
fn test_cvlr_lemma_apply() {
    cvlr_lemma! {
        XAndYPositiveLemma(c: TestCtx) {
            requires -> {
                c.x > 0
            }
            ensures -> {
                c.x > 0;
                c.y > 0
            }
        }
    }

    let lemma = XAndYPositiveLemma;

    // Test apply - should assume requires and assert ensures
    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };

    // This should not panic since both requires and ensures hold
    lemma.apply(&ctx);
}

#[test]
fn test_cvlr_lemma_multiple_conditions() {
    cvlr_lemma! {
        ComplexLemma(c: TestCtx) {
            requires -> {
                c.x > 0;
                c.y > 0;
                c.flag
            }
            ensures -> {
                c.x > 0;
                c.y > 0;
                c.x + c.y > 10
            }
        }
    }

    let lemma = ComplexLemma;

    let ctx1 = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };

    // Test requires
    assert!(lemma.requires().eval(&ctx1));

    let ctx2 = TestCtx {
        x: 5,
        y: 10,
        flag: false,
    };
    assert!(!lemma.requires().eval(&ctx2));

    // Test ensures
    assert!(lemma.ensures().eval(&ctx1));

    let ctx3 = TestCtx {
        x: 1,
        y: 2,
        flag: true,
    };
    assert!(!lemma.ensures().eval(&ctx3));
}

// Manual implementation of CvlrLemma for testing
struct ManualLemma;

impl cvlr_spec::spec::CvlrLemma for ManualLemma {
    type Context = TestCtx;
    fn requires(&self) -> impl CvlrFormula<Context = Self::Context> {
        cvlr_predicate! { | c : TestCtx | -> {
            c.x > 0
        } }
    }

    fn ensures(&self) -> impl CvlrFormula<Context = Self::Context> {
        cvlr_predicate! { | c : TestCtx | -> {
            c.x > 0;
            c.y > 0
        } }
    }
}

#[test]
fn test_cvlr_lemma_trait_manual_impl() {
    let lemma = ManualLemma;

    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };

    // Test requires
    assert!(lemma.requires().eval(&ctx));

    // Test ensures
    assert!(lemma.ensures().eval(&ctx));

    // Test verify_with_context
    lemma.verify_with_context(&ctx);

    // Test apply
    lemma.apply(&ctx);
}

#[test]
fn test_cvlr_lemma_requires_ensures_interaction() {
    cvlr_lemma! {
        ImplicationLemma(c: TestCtx) {
            requires -> {
                c.x > 0
            }
            ensures -> {
                c.x > 0;
                c.y == c.x * 2
            }
        }
    }

    let lemma = ImplicationLemma;

    // Test that requires can be true while ensures is false
    let ctx1 = TestCtx {
        x: 5,
        y: 5, // y != x * 2
        flag: false,
    };

    assert!(lemma.requires().eval(&ctx1));
    assert!(!lemma.ensures().eval(&ctx1));

    // Test that both can be true
    let ctx2 = TestCtx {
        x: 5,
        y: 10, // y == x * 2
        flag: false,
    };

    assert!(lemma.requires().eval(&ctx2));
    assert!(lemma.ensures().eval(&ctx2));
}

#[test]
fn test_eval_with_states() {
    // Test that eval_with_states works for single-state predicates
    let pre = TestCtx {
        x: 1,
        y: 2,
        flag: false,
    };
    let post = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };

    // Test with XPositive - should only check post.x > 0
    let x_positive = XPositive;
    assert!(x_positive.eval_with_states(&post, &pre)); // post.x = 5 > 0

    // Test with YPositive - should only check post.y > 0
    let y_positive = YPositive;
    assert!(y_positive.eval_with_states(&post, &pre)); // post.y = 10 > 0

    // Test that it ignores pre-state - even if pre.x is negative, it should pass
    let pre2 = TestCtx {
        x: -10,
        y: -5,
        flag: false,
    };
    let post2 = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    assert!(x_positive.eval_with_states(&post2, &pre2)); // post.x = 5 > 0, ignores pre.x = -10
    assert!(y_positive.eval_with_states(&post2, &pre2)); // post.y = 10 > 0, ignores pre.y = -5

    // Test with negative post-state - should fail even if pre-state is positive
    let pre3 = TestCtx {
        x: 10,
        y: 20,
        flag: true,
    };
    let post3 = TestCtx {
        x: -5,
        y: -10,
        flag: false,
    };
    assert!(!x_positive.eval_with_states(&post3, &pre3)); // post.x = -5 <= 0
    assert!(!y_positive.eval_with_states(&post3, &pre3)); // post.y = -10 <= 0
}

#[test]
fn test_eval_with_states_with_cvlr_true() {
    // Test that cvlr_true works with eval_with_states
    let pre = TestCtx {
        x: 1,
        y: 2,
        flag: false,
    };
    let post = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };

    let true_expr = cvlr_true::<TestCtx>();
    assert!(true_expr.eval_with_states(&post, &pre)); // cvlr_true always evaluates to true
}

#[test]
fn test_eval_with_states_with_composed_expressions() {
    // Test eval_with_states with composed expressions
    let pre = TestCtx {
        x: 1,
        y: 2,
        flag: false,
    };
    let post = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };

    // Test with AND expression
    let and_expr = cvlr_and(XPositive, YPositive);
    assert!(and_expr.eval_with_states(&post, &pre)); // Both post.x > 0 and post.y > 0

    // Test with negative case
    let post2 = TestCtx {
        x: -5,
        y: 10,
        flag: false,
    };
    assert!(!and_expr.eval_with_states(&post2, &pre)); // post.x = -5 <= 0

    // Test with implication
    let impl_expr = cvlr_implies(XPositive, YPositive);
    assert!(impl_expr.eval_with_states(&post, &pre)); // post.x > 0 -> post.y > 0 (both true)

    let post3 = TestCtx {
        x: 5,
        y: -10,
        flag: false,
    };
    assert!(!impl_expr.eval_with_states(&post3, &pre)); // post.x > 0 -> post.y > 0 (antecedent true, consequent false)
}

#[test]
fn test_cvlr_spec_macro() {
    // Test cvlr_spec! macro creates a spec correctly
    let spec = cvlr_spec! {
        requires: XPositive,
        ensures: YPositive
    };

    let pre = TestCtx {
        x: 5,
        y: 0,
        flag: false,
    };
    let post = TestCtx {
        x: 5,
        y: 10,
        flag: false,
    };

    // Test assume_requires
    spec.assume_requires(&pre); // Should assume XPositive holds for pre

    // Test check_ensures
    spec.check_ensures(&post, &pre); // Should assert YPositive holds for post
}

#[test]
fn test_cvlr_spec_macro_with_predicates() {
    // Test cvlr_spec! macro with cvlr_predicate!
    let spec = cvlr_spec! {
        requires: cvlr_predicate! { | c : TestCtx | -> { c.x > 0 } },
        ensures: cvlr_predicate! { | c : TestCtx | -> { c.y > 0 } }
    };

    let pre = TestCtx {
        x: 5,
        y: 0,
        flag: false,
    };
    let post = TestCtx {
        x: 5,
        y: 10,
        flag: false,
    };

    spec.assume_requires(&pre);
    spec.check_ensures(&post, &pre);
}

#[test]
fn test_cvlr_invar_spec_macro() {
    // Test cvlr_invar_spec! macro creates an invariant spec correctly
    let spec = cvlr_invar_spec! {
        assumption: XPositive,
        invariant: YPositive
    };

    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: false,
    };

    // Test assume_requires - should assume both assumption and invariant
    spec.assume_requires(&ctx);

    // Test check_ensures - should assert invariant holds
    spec.check_ensures(&ctx, &ctx);
}

#[test]
fn test_cvlr_invar_spec_macro_with_predicates() {
    // Test cvlr_invar_spec! macro with cvlr_predicate!
    let spec = cvlr_invar_spec! {
        assumption: cvlr_predicate! { | c : TestCtx | -> { c.x > 0 } },
        invariant: cvlr_predicate! { | c : TestCtx | -> { c.y > 0 } }
    };

    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: false,
    };

    spec.assume_requires(&ctx);
    spec.check_ensures(&ctx, &ctx);
}

// Mock macro for testing cvlr_rules! and cvlr_invariant_rules!
// In real usage, this would be provided by the user
macro_rules! cvlr_impl_rule {
    {$rule_name:ident, $spec:expr, $base:ident} => {
        // Just verify the macro expands correctly
        {
            let _rule_name = stringify!($rule_name);
            let _spec = $spec;
            let _base = stringify!($base);
        }
    };
}

#[test]
fn test_cvlr_rules_macro() {
    // Test cvlr_rules! macro expands correctly
    // Define rules for multiple functions
    cvlr_rules! {
        name: "solvency",
        spec: cvlr_spec! {
            requires: XPositive,
            ensures: YPositive
        },
        bases: [
            base_update_counter,
            base_reset_counter,
            base_increment_counter,
        ]
    }

    // The macro should expand to three calls to cvlr_rule_for_spec!
    // We can't directly test the expansion, but we can verify it compiles
}

#[test]
fn test_cvlr_rules_macro_without_base_prefix() {
    // Test cvlr_rules! with functions that don't have base_ prefix
    cvlr_rules! {
        name: "liquidity",
        spec: cvlr_spec! {
            requires: XPositive,
            ensures: YPositive
        },
        bases: [
            update_function,
            reset_function,
        ]
    }
}

#[test]
fn test_cvlr_rules_macro_single_base() {
    // Test cvlr_rules! with a single base function
    cvlr_rules! {
        name: "test_rule",
        spec: cvlr_spec! {
            requires: XPositive,
            ensures: YPositive
        },
        bases: [
            base_single_function,
        ]
    }
}

#[test]
fn test_cvlr_invariant_rules_macro() {
    // Test cvlr_invariant_rules! macro expands correctly
    // Define invariant rules for multiple functions
    cvlr_invariant_rules! {
        name: "non_negative",
        assumption: cvlr_predicate! { | c : TestCtx | -> { c.x > 0 } },
        invariant: cvlr_predicate! { | c : TestCtx | -> { c.y >= 0 } },
        bases: [
            base_update_counter,
            base_reset_counter,
            base_increment_counter,
        ]
    }

    // The macro should expand to three calls to cvlr_rule_for_spec!
    // with an invariant spec created from assumption and invariant
}

#[test]
fn test_cvlr_invariant_rules_macro_with_simple_expressions() {
    // Test cvlr_invariant_rules! with simple boolean expressions
    cvlr_invariant_rules! {
        name: "positive",
        assumption: XPositive,
        invariant: YPositive,
        bases: [
            base_function1,
            base_function2,
        ]
    }
}

#[test]
fn test_cvlr_invariant_rules_macro_single_base() {
    // Test cvlr_invariant_rules! with a single base function
    cvlr_invariant_rules! {
        name: "test_invariant",
        assumption: XPositive,
        invariant: YPositive,
        bases: [
            base_single_function,
        ]
    }
}

// Tests for cvlr_and! macro
#[test]
fn test_cvlr_and_macro_with_identifiers() {
    // Test cvlr_and! macro with identifier arguments
    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    let expr = cvlr_and!(XPositive, YPositive);
    assert!(expr.eval(&ctx));

    let ctx2 = TestCtx {
        x: -1,
        y: 10,
        flag: true,
    };
    assert!(!expr.eval(&ctx2));
}

#[test]
fn test_cvlr_and_macro_with_expressions() {
    // Test cvlr_and! macro with expression arguments
    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    let expr = cvlr_and!(
        cvlr_predicate! { | c : TestCtx | -> { c.x > 0 } },
        cvlr_predicate! { | c : TestCtx | -> { c.y > 0 } }
    );
    assert!(expr.eval(&ctx));

    let ctx2 = TestCtx {
        x: -1,
        y: 10,
        flag: true,
    };
    assert!(!expr.eval(&ctx2));
}

#[test]
fn test_cvlr_and_macro_mixed_ident_expr() {
    // Test cvlr_and! macro with mixed identifier and expression arguments
    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    let expr1 = cvlr_and!(
        XPositive,
        cvlr_predicate! { | c : TestCtx | -> { c.y > 0 } }
    );
    assert!(expr1.eval(&ctx));

    let expr2 = cvlr_and!(
        cvlr_predicate! { | c : TestCtx | -> { c.x > 0 } },
        YPositive
    );
    assert!(expr2.eval(&ctx));
}

#[test]
fn test_cvlr_and_macro_three_args() {
    // Test cvlr_and! macro with three arguments
    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    let true_expr = cvlr_true::<TestCtx>();
    let expr = cvlr_and!(XPositive, YPositive, true_expr);
    assert!(expr.eval(&ctx));

    let ctx2 = TestCtx {
        x: -1,
        y: 10,
        flag: true,
    };
    assert!(!expr.eval(&ctx2));
}

#[test]
fn test_cvlr_and_macro_four_args() {
    // Test cvlr_and! macro with four arguments
    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    let true_expr = cvlr_true::<TestCtx>();
    let expr = cvlr_and!(
        XPositive,
        YPositive,
        true_expr,
        cvlr_predicate! { | c : TestCtx | -> { c.flag } }
    );
    assert!(expr.eval(&ctx));

    let ctx2 = TestCtx {
        x: 5,
        y: 10,
        flag: false,
    };
    assert!(!expr.eval(&ctx2));
}

#[test]
fn test_cvlr_and_macro_five_args() {
    // Test cvlr_and! macro with five arguments
    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    let true_expr = cvlr_true::<TestCtx>();
    let expr = cvlr_and!(
        XPositive,
        YPositive,
        true_expr,
        cvlr_predicate! { | c : TestCtx | -> { c.flag } },
        cvlr_predicate! { | c : TestCtx | -> { c.x + c.y > 0 } }
    );
    assert!(expr.eval(&ctx));

    let ctx2 = TestCtx {
        x: -5,
        y: -10,
        flag: true,
    };
    assert!(!expr.eval(&ctx2));
}

#[test]
fn test_cvlr_and_macro_six_args() {
    // Test cvlr_and! macro with six arguments
    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    let true_expr = cvlr_true::<TestCtx>();
    let expr = cvlr_and!(
        XPositive,
        YPositive,
        true_expr,
        cvlr_predicate! { | c : TestCtx | -> { c.flag } },
        cvlr_predicate! { | c : TestCtx | -> { c.x + c.y > 0 } },
        cvlr_predicate! { | c : TestCtx | -> { c.x * c.y > 0 } }
    );
    assert!(expr.eval(&ctx));

    let ctx2 = TestCtx {
        x: 0,
        y: 10,
        flag: true,
    };
    assert!(!expr.eval(&ctx2));
}

#[test]
fn test_cvlr_and_macro_with_assert() {
    // Test that cvlr_and! macro works with assert
    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    let expr = cvlr_and!(XPositive, YPositive);
    expr.assert(&ctx); // Should not panic
}

#[test]
fn test_cvlr_and_macro_with_assume() {
    // Test that cvlr_and! macro works with assume
    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    let expr = cvlr_and!(XPositive, YPositive);
    expr.assume(&ctx); // Should not panic
}

#[test]
fn test_cvlr_and_macro_with_eval_with_states() {
    // Test that cvlr_and! macro works with eval_with_states
    let pre = TestCtx {
        x: 1,
        y: 2,
        flag: false,
    };
    let post = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    let expr = cvlr_and!(XPositive, YPositive);
    assert!(expr.eval_with_states(&post, &pre));

    let post2 = TestCtx {
        x: -5,
        y: 10,
        flag: false,
    };
    assert!(!expr.eval_with_states(&post2, &pre));
}

// Tests for cvlr_implies! macro
#[test]
fn test_cvlr_implies_macro_with_identifiers() {
    // Test cvlr_implies! macro with identifier arguments
    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    // x > 0 -> y > 0 (both true, so true)
    let expr = cvlr_implies!(XPositive, YPositive);
    assert!(expr.eval(&ctx));

    // x > 0 -> y > 0 (antecedent true, consequent false, so false)
    let ctx2 = TestCtx {
        x: 5,
        y: -1,
        flag: true,
    };
    assert!(!expr.eval(&ctx2));

    // x > 0 -> y > 0 (antecedent false, so true regardless of consequent)
    let ctx3 = TestCtx {
        x: -1,
        y: -1,
        flag: true,
    };
    assert!(expr.eval(&ctx3));

    // x > 0 -> y > 0 (antecedent false, consequent true, so true)
    let ctx4 = TestCtx {
        x: -1,
        y: 10,
        flag: true,
    };
    assert!(expr.eval(&ctx4));
}

#[test]
fn test_cvlr_implies_macro_with_expressions() {
    // Test cvlr_implies! macro with expression arguments
    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    let expr = cvlr_implies!(
        cvlr_predicate! { | c : TestCtx | -> { c.x > 0 } },
        cvlr_predicate! { | c : TestCtx | -> { c.y > 0 } }
    );
    assert!(expr.eval(&ctx));

    let ctx2 = TestCtx {
        x: 5,
        y: -1,
        flag: true,
    };
    assert!(!expr.eval(&ctx2));
}

#[test]
fn test_cvlr_implies_macro_mixed_ident_expr() {
    // Test cvlr_implies! macro with mixed identifier and expression arguments
    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    // Identifier as antecedent, expression as consequent
    let expr1 = cvlr_implies!(
        XPositive,
        cvlr_predicate! { | c : TestCtx | -> { c.y > 0 } }
    );
    assert!(expr1.eval(&ctx));

    // Expression as antecedent, identifier as consequent
    let expr2 = cvlr_implies!(
        cvlr_predicate! { | c : TestCtx | -> { c.x > 0 } },
        YPositive
    );
    assert!(expr2.eval(&ctx));

    // Test false case
    let ctx2 = TestCtx {
        x: 5,
        y: -1,
        flag: true,
    };
    assert!(!expr1.eval(&ctx2));
    assert!(!expr2.eval(&ctx2));
}

#[test]
fn test_cvlr_implies_macro_with_assert() {
    // Test that cvlr_implies! macro works with assert
    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    let expr = cvlr_implies!(XPositive, YPositive);
    expr.assert(&ctx); // Should not panic (antecedent true, consequent true)

    // When antecedent is false, assert should not panic (consequent not checked)
    let ctx2 = TestCtx {
        x: -1,
        y: -1,
        flag: true,
    };
    expr.assert(&ctx2); // Should not panic (antecedent false, consequent not checked)
}

#[test]
fn test_cvlr_implies_macro_with_assume() {
    // Test that cvlr_implies! macro works with assume
    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    let expr = cvlr_implies!(XPositive, YPositive);
    expr.assume(&ctx); // Should not panic

    // When antecedent is false, assume should not panic (consequent not checked)
    let ctx2 = TestCtx {
        x: -1,
        y: -1,
        flag: true,
    };
    expr.assume(&ctx2); // Should not panic
}

#[test]
fn test_cvlr_implies_macro_with_eval_with_states() {
    // Test that cvlr_implies! macro works with eval_with_states
    let pre = TestCtx {
        x: 1,
        y: 2,
        flag: false,
    };
    let post = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    let expr = cvlr_implies!(XPositive, YPositive);
    assert!(expr.eval_with_states(&post, &pre)); // post.x > 0 -> post.y > 0 (both true)

    let post2 = TestCtx {
        x: 5,
        y: -10,
        flag: false,
    };
    assert!(!expr.eval_with_states(&post2, &pre)); // post.x > 0 -> post.y > 0 (antecedent true, consequent false)

    let post3 = TestCtx {
        x: -5,
        y: -10,
        flag: false,
    };
    assert!(expr.eval_with_states(&post3, &pre)); // post.x > 0 -> ... (antecedent false, so true)
}

#[test]
fn test_cvlr_and_and_implies_macro_composition() {
    // Test composing cvlr_and! and cvlr_implies! macros
    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    // (x > 0 -> y > 0) && (y > 0 -> x > 0)
    let expr = cvlr_and!(
        cvlr_implies!(XPositive, YPositive),
        cvlr_implies!(YPositive, XPositive)
    );
    assert!(expr.eval(&ctx));

    // Test with one implication false
    let ctx2 = TestCtx {
        x: 5,
        y: -1,
        flag: true,
    };
    assert!(!expr.eval(&ctx2));
}

#[test]
fn test_cvlr_and_macro_nested() {
    // Test nested cvlr_and! macro calls
    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    let inner = cvlr_and!(XPositive, YPositive);
    let outer = cvlr_and!(inner, cvlr_predicate! { | c : TestCtx | -> { c.flag } });
    assert!(outer.eval(&ctx));

    let ctx2 = TestCtx {
        x: 5,
        y: 10,
        flag: false,
    };
    assert!(!outer.eval(&ctx2));
}

// Tests for the new cvlr_lemma! branch with name literal syntax
#[test]
fn test_cvlr_lemma_new_branch_basic() {
    // Test the new branch syntax: name: "string", StructName(Context) { requires: expr, ensures: expr }
    cvlr_lemma! {
        name: "BasicLemma",
        BasicLemmaNew(TestCtx) {
            requires: cvlr_predicate! { | c : TestCtx | -> { c.x > 0 } },
            ensures: cvlr_predicate! { | c : TestCtx | -> { c.x > 0; c.y >= 0 } },
        }
    }

    let lemma = BasicLemmaNew;

    let ctx1 = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    assert!(lemma.requires().eval(&ctx1));
    assert!(lemma.ensures().eval(&ctx1));

    let ctx2 = TestCtx {
        x: -1,
        y: 10,
        flag: true,
    };
    assert!(!lemma.requires().eval(&ctx2));

    let ctx3 = TestCtx {
        x: 5,
        y: -1,
        flag: true,
    };
    assert!(!lemma.ensures().eval(&ctx3));
}

#[test]
fn test_cvlr_lemma_new_branch_with_identifiers() {
    // Test the new branch with identifier predicates
    cvlr_lemma! {
        name: "IdentifierLemma",
        IdentifierLemmaNew(TestCtx) {
            requires: XPositive,
            ensures: YPositive,
        }
    }

    let lemma = IdentifierLemmaNew;

    let ctx1 = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    assert!(lemma.requires().eval(&ctx1));
    assert!(lemma.ensures().eval(&ctx1));

    let ctx2 = TestCtx {
        x: -1,
        y: 10,
        flag: true,
    };
    assert!(!lemma.requires().eval(&ctx2));
    assert!(lemma.ensures().eval(&ctx2));

    let ctx3 = TestCtx {
        x: 5,
        y: -1,
        flag: true,
    };
    assert!(lemma.requires().eval(&ctx3));
    assert!(!lemma.ensures().eval(&ctx3));
}

#[test]
fn test_cvlr_lemma_new_branch_with_composed_expressions() {
    // Test the new branch with composed expressions (cvlr_and, cvlr_implies)
    cvlr_lemma! {
        name: "ComposedLemma",
        ComposedLemmaNew(TestCtx) {
            requires: cvlr_and!(XPositive, YPositive),
            ensures: cvlr_implies!(XPositive, YPositive),
        }
    }

    let lemma = ComposedLemmaNew;

    let ctx1 = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    assert!(lemma.requires().eval(&ctx1));
    assert!(lemma.ensures().eval(&ctx1));

    let ctx2 = TestCtx {
        x: -1,
        y: 10,
        flag: true,
    };
    assert!(!lemma.requires().eval(&ctx2));
    assert!(lemma.ensures().eval(&ctx2)); // antecedent false, so implication is true

    let ctx3 = TestCtx {
        x: 5,
        y: -1,
        flag: true,
    };
    assert!(!lemma.requires().eval(&ctx3));
    assert!(!lemma.ensures().eval(&ctx3)); // antecedent true, consequent false, so false
}

#[test]
fn test_cvlr_lemma_new_branch_verify_with_context() {
    // Test verify_with_context with the new branch
    cvlr_lemma! {
        name: "VerifyLemma",
        VerifyLemmaNew(TestCtx) {
            requires: cvlr_predicate! { | c : TestCtx | -> { c.x > 0 } },
            ensures: cvlr_predicate! { | c : TestCtx | -> { c.x > 0 } },
        }
    }

    let lemma = VerifyLemmaNew;

    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };

    // This should not panic since requires and ensures both hold
    lemma.verify_with_context(&ctx);
}

#[test]
fn test_cvlr_lemma_new_branch_apply() {
    // Test apply with the new branch
    cvlr_lemma! {
        name: "ApplyLemma",
        ApplyLemmaNew(TestCtx) {
            requires: XPositive,
            ensures: cvlr_and!(XPositive, YPositive),
        }
    }

    let lemma = ApplyLemmaNew;

    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };

    // This should not panic since both requires and ensures hold
    lemma.apply(&ctx);
}

#[test]
fn test_cvlr_lemma_new_branch_multiple_conditions() {
    // Test the new branch with multiple conditions using cvlr_and
    cvlr_lemma! {
        name: "MultipleConditionsLemma",
        MultipleConditionsLemmaNew(TestCtx) {
            requires: cvlr_and!(
                XPositive,
                YPositive,
                cvlr_predicate! { | c : TestCtx | -> { c.flag } }
            ),
            ensures: cvlr_and!(
                XPositive,
                YPositive,
                cvlr_predicate! { | c : TestCtx | -> { c.x + c.y > 10 } }
            ),
        }
    }

    let lemma = MultipleConditionsLemmaNew;

    let ctx1 = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    assert!(lemma.requires().eval(&ctx1));
    assert!(lemma.ensures().eval(&ctx1));

    let ctx2 = TestCtx {
        x: 5,
        y: 10,
        flag: false,
    };
    assert!(!lemma.requires().eval(&ctx2));
    assert!(lemma.ensures().eval(&ctx2));

    let ctx3 = TestCtx {
        x: 1,
        y: 2,
        flag: true,
    };
    assert!(lemma.requires().eval(&ctx3));
    assert!(!lemma.ensures().eval(&ctx3)); // x + y = 3 <= 10
}

#[test]
fn test_cvlr_lemma_new_branch_mixed_expressions() {
    // Test the new branch with mixed identifier and predicate expressions
    cvlr_lemma! {
        name: "MixedExpressionsLemma",
        MixedExpressionsLemmaNew(TestCtx) {
            requires: cvlr_and!(
                XPositive,
                cvlr_predicate! { | c : TestCtx | -> { c.y > 0 } }
            ),
            ensures: cvlr_implies!(
                YPositive,
                cvlr_predicate! { | c : TestCtx | -> { c.x > 0 } }
            ),
        }
    }

    let lemma = MixedExpressionsLemmaNew;

    let ctx1 = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    assert!(lemma.requires().eval(&ctx1));
    assert!(lemma.ensures().eval(&ctx1));

    let ctx2 = TestCtx {
        x: -1,
        y: 10,
        flag: true,
    };
    assert!(!lemma.requires().eval(&ctx2));
    assert!(!lemma.ensures().eval(&ctx2));
    assert!(!lemma.ensures().eval(&ctx2));
}

#[test]
fn test_cvlr_lemma_new_branch_with_trailing_comma() {
    // Test the new branch with trailing comma
    cvlr_lemma! {
        name: "TrailingCommaLemma",
        TrailingCommaLemmaNew(TestCtx) {
            requires: XPositive,
            ensures: YPositive,
        }
    }

    let lemma = TrailingCommaLemmaNew;

    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    assert!(lemma.requires().eval(&ctx));
    assert!(lemma.ensures().eval(&ctx));
}

#[test]
fn test_cvlr_lemma_new_branch_requires_ensures_interaction() {
    // Test that requires and ensures can be independent in the new branch
    cvlr_lemma! {
        name: "InteractionLemma",
        InteractionLemmaNew(TestCtx) {
            requires: cvlr_predicate! { | c : TestCtx | -> { c.x > 0 } },
            ensures: cvlr_predicate! { | c : TestCtx | -> { c.x > 0; c.y == c.x * 2 } },
        }
    }

    let lemma = InteractionLemmaNew;

    // Test that requires can be true while ensures is false
    let ctx1 = TestCtx {
        x: 5,
        y: 5, // y != x * 2
        flag: false,
    };
    assert!(lemma.requires().eval(&ctx1));
    assert!(!lemma.ensures().eval(&ctx1));

    // Test that both can be true
    let ctx2 = TestCtx {
        x: 5,
        y: 10, // y == x * 2
        flag: false,
    };
    assert!(lemma.requires().eval(&ctx2));
    assert!(lemma.ensures().eval(&ctx2));
}
