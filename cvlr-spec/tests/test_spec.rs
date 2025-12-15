//! Tests for cvlr-spec crate

extern crate cvlr;

use cvlr_spec::*;

// Test context type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

// Boolean expression for StatePair that checks if post.y > 0
#[derive(Copy, Clone)]
struct PostYPositive;

impl CvlrBoolExpr<TestCtx> for XPositive {
    fn eval(&self, ctx: &TestCtx) -> bool {
        ctx.x > 0
    }
}

impl CvlrBoolExpr<TestCtx> for YPositive {
    fn eval(&self, ctx: &TestCtx) -> bool {
        ctx.y > 0
    }
}

impl<'a> CvlrBoolExpr<StatePair<'a, TestCtx>> for PostYPositive {
    fn eval(&self, pair: &StatePair<'a, TestCtx>) -> bool {
        pair.ctx().y > 0
    }
}

#[test]
fn test_cvlr_true() {
    let ctx = TestCtx {
        x: 0,
        y: 0,
        flag: false,
    };
    let true_expr = CvlrTrue;
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
    let true_expr = CvlrTrue;
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
    let pair = StatePair::new(&ctx1, &ctx2);

    assert_eq!(pair.ctx(), &ctx1);
    assert_eq!(pair.old(), &ctx2);
    assert_eq!(pair.pre(), &ctx2);
    assert_eq!(pair.post(), &ctx1);
}

#[test]
fn test_state_pair_singleton() {
    let ctx = TestCtx {
        x: 1,
        y: 2,
        flag: true,
    };
    let pair = StatePair::singleton(&ctx);

    assert_eq!(pair.ctx(), &ctx);
    assert_eq!(pair.old(), &ctx);
    assert_eq!(pair.pre(), &ctx);
    assert_eq!(pair.post(), &ctx);
}

#[test]
fn test_state_pair_deref() {
    let ctx = TestCtx {
        x: 1,
        y: 2,
        flag: true,
    };
    let pair = StatePair::singleton(&ctx);

    // Test Deref implementation
    assert_eq!(pair.x, 1);
    assert_eq!(pair.y, 2);
    assert_eq!(pair.flag, true);
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
    let pair = StatePair::new(&post, &pre);

    let pred = XIncreased;
    assert!(pred.eval(&pair));

    let pair2 = StatePair::new(&pre, &post);
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
    let pair = StatePair::new(&post, &pre);

    let pred = XAndYIncreased;
    assert!(pred.eval(&pair));

    let post2 = TestCtx {
        x: 5,
        y: 1,
        flag: false,
    };
    let pair2 = StatePair::new(&post2, &pre);
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
    let pair = StatePair::new(&post, &pre);
    spec.check_ensures(pair);
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
    // Test that cvlr_impl_statepair preserves HRTB bounds
    let requires = XPositive;
    let ensures = cvlr_impl_pair(PostXPositive, PostYPositive);

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
    let pair = StatePair::new(&post, &pre);
    spec.check_ensures(pair);
}

#[test]
fn test_cvlr_spec_with_and() {
    // Test that cvlr_and_statepair preserves HRTB bounds
    let requires = XPositive;
    let ensures = cvlr_and_pair(PostXPositive, PostYPositive);

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
    let pair = StatePair::new(&post, &pre);
    spec.check_ensures(pair);
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
    let pair = StatePair::new(&post, &pre);
    spec.check_ensures(pair);
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
    let true_expr = CvlrTrue;

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
    let expr1 = cvlr_and(cvlr_and(XPositive, YPositive), CvlrTrue);
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
        let pair = StatePair::new(&ctx1, &ctx2);
        assert_eq!(pair.ctx().x, 1);
        assert_eq!(pair.old().x, 3);
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
    let pair = StatePair::new(&post, &pre);

    assert!(XIncreased.eval(&pair));
    assert!(YIncreased.eval(&pair));
    assert!(BothIncreased.eval(&pair));

    let post2 = TestCtx {
        x: 5,
        y: 1,
        flag: false,
    };
    let pair2 = StatePair::new(&post2, &pre);
    assert!(XIncreased.eval(&pair2));
    assert!(!YIncreased.eval(&pair2));
    assert!(!BothIncreased.eval(&pair2));
}
