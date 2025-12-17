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

impl CvlrBoolExpr for XPositive {
    type Context = TestCtx;
    fn eval(&self, ctx: &Self::Context) -> bool {
        ctx.x > 0
    }
}

impl CvlrBoolExpr for YPositive {
    type Context = TestCtx;
    fn eval(&self, ctx: &Self::Context) -> bool {
        ctx.y > 0
    }
}

impl CvlrBoolExpr for PostYPositive {
    type Context = (TestCtx, TestCtx);
    fn eval(&self, pair: &Self::Context) -> bool {
        pair.0.y > 0
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
fn test_cvlr_impl() {
    let ctx = TestCtx {
        x: 5,
        y: 10,
        flag: true,
    };
    // x > 0 -> y > 0 (both true, so true)
    let impl_expr = cvlr_impl(XPositive, YPositive);
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

cvlr_def_state_pair_predicate! {
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
    let pair = (post, pre);

    let pred = XIncreased;
    assert!(pred.eval(&pair));

    let pair2 = (pre, post);
    assert!(!pred.eval(&pair2));
}

cvlr_def_state_pair_predicate! {
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
    let pair = (post, pre);

    let pred = XAndYIncreased;
    assert!(pred.eval(&pair));

    let post2 = TestCtx {
        x: 5,
        y: 1,
        flag: false,
    };
    let pair2 = (post2, pre);
    assert!(!pred.eval(&pair2));
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
    let pair = (post, pre);
    spec.check_ensures(&pair);
}
// Define predicates for the ensures condition
cvlr_def_state_pair_predicate! {
    pred PostXPositive([ c, o ] : TestCtx) {
        c.x > 0
    }
}

#[test]
fn test_cvlr_spec_with_implication() {
    cvlr_def_state_pair_predicate! {
        pred PostYPositive([ c, o ] : TestCtx) {
            c.y > 0
        }
    }

    // Create a spec: requires x > 0, ensures if x > 0 then y > 0
    // Test that cvlr_impl preserves HRTB bounds
    let requires = XPositive;
    let ensures = cvlr_impl(PostXPositive, PostYPositive);

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
    let pair = (post, pre);
    spec.check_ensures(&pair);
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
    let pair = (post, pre);
    spec.check_ensures(&pair);
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
    let pair = (post, pre);
    spec.check_ensures(&pair);
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
        cvlr_impl(XPositive, YPositive),
        cvlr_impl(YPositive, XPositive),
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
fn test_cvlr_def_state_pair_predicates() {
    cvlr_def_state_pair_predicates! {
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
    let pair = (post, pre);

    assert!(XIncreased.eval(&pair));
    assert!(YIncreased.eval(&pair));
    assert!(BothIncreased.eval(&pair));

    let post2 = TestCtx {
        x: 5,
        y: 1,
        flag: false,
    };
    let pair2 = (post2, pre);
    assert!(XIncreased.eval(&pair2));
    assert!(!YIncreased.eval(&pair2));
    assert!(!BothIncreased.eval(&pair2));
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
    fn requires(&self) -> impl CvlrBoolExpr<Context = Self::Context> {
        cvlr_predicate! { | c : TestCtx | -> {
            c.x > 0
        } }
    }

    fn ensures(&self) -> impl CvlrBoolExpr<Context = Self::Context> {
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
fn test_to_two_state() {
    // Test converting a boolean expression over TestCtx to one over (TestCtx, TestCtx) tuple
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
    let pair = (post, pre);

    // Test with XPositive - should only check post.x > 0
    let x_positive = XPositive;
    let x_positive_state_pair = x_positive.to_two_state();
    assert!(x_positive_state_pair.eval(&pair)); // post.x = 5 > 0

    // Test with YPositive - should only check post.y > 0
    let y_positive = YPositive;
    let y_positive_state_pair = y_positive.to_two_state();
    assert!(y_positive_state_pair.eval(&pair)); // post.y = 10 > 0

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
    let pair2 = (post2, pre2);
    assert!(x_positive_state_pair.eval(&pair2)); // post.x = 5 > 0, ignores pre.x = -10
    assert!(y_positive_state_pair.eval(&pair2)); // post.y = 10 > 0, ignores pre.y = -5

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
    let pair3 = (post3, pre3);
    assert!(!x_positive_state_pair.eval(&pair3)); // post.x = -5 <= 0
    assert!(!y_positive_state_pair.eval(&pair3)); // post.y = -10 <= 0
}

#[test]
fn test_to_two_state_with_cvlr_true() {
    // Test that CvlrTrue works with to_two_state
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
    let pair = (post, pre);

    let true_expr = cvlr_true::<TestCtx>();
    let true_state_pair = true_expr.to_two_state();
    assert!(true_state_pair.eval(&pair)); // CvlrTrue always evaluates to true
}

#[test]
fn test_to_two_state_with_composed_expressions() {
    // Test converting composed expressions
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
    let pair = (post, pre);

    // Test with AND expression
    let and_expr = cvlr_and(XPositive, YPositive);
    let and_state_pair = and_expr.to_two_state();
    assert!(and_state_pair.eval(&pair)); // Both post.x > 0 and post.y > 0

    // Test with negative case
    let post2 = TestCtx {
        x: -5,
        y: 10,
        flag: false,
    };
    let pair2 = (post2, pre);
    assert!(!and_state_pair.eval(&pair2)); // post.x = -5 <= 0

    // Test with implication
    let impl_expr = cvlr_impl(XPositive, YPositive);
    let impl_state_pair = impl_expr.to_two_state();
    assert!(impl_state_pair.eval(&pair)); // post.x > 0 -> post.y > 0 (both true)

    let post3 = TestCtx {
        x: 5,
        y: -10,
        flag: false,
    };
    let pair3 = (post3, pre);
    assert!(!impl_state_pair.eval(&pair3)); // post.x > 0 -> post.y > 0 (antecedent true, consequent false)
}
