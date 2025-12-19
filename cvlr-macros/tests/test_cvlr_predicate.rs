//! Runtime tests for cvlr_predicate macro

use cvlr::spec::CvlrFormula;
use cvlr_macros::cvlr_predicate;

pub struct Ctx {
    x: i32,
    y: i32,
}

#[cvlr_predicate]
pub fn x_gt_zero(c: &Ctx) {
    c.x > 0;
}

#[cvlr_predicate]
fn y_lt_hundred(c: &Ctx) {
    c.y < 100;
}

#[cvlr_predicate]
fn multiple_conditions(c: &Ctx) {
    c.x > 0;
    c.y < 100;
}

#[cvlr_predicate]
fn with_let_statement(c: &Ctx) {
    let threshold = 0;
    c.x > threshold;
}

#[cvlr_predicate]
fn with_multiple_lets(c: &Ctx) {
    let min_x = 0;
    let max_y = 100;
    c.x > min_x;
    c.y < max_y;
}

#[cvlr_predicate]
fn let_before_expressions(c: &Ctx) {
    let threshold = 5;
    let limit = 100;
    c.x > threshold;
    c.y < limit;
    c.x + c.y > threshold;
}

#[cvlr_predicate]
fn with_if_else(c: &Ctx) {
    if c.x > 0 {
        c.y > 0
    } else {
        c.y < 0
    };
}

#[cvlr_predicate]
fn with_if_else_true(c: &Ctx) {
    if c.x > 0 {
        c.y > 0
    } else {
        true
    };
}

#[cvlr_predicate]
fn with_if_else_both_true(c: &Ctx) {
    if c.x > 0 {
        true
    } else {
        true
    };
}

#[cvlr_predicate]
fn with_nested_if_else(c: &Ctx) {
    if c.x > 0 {
        if c.y > 0 {
            c.x + c.y > 0
        } else {
            c.x > c.y
        }
    } else {
        c.y < 0
    };
}

#[cvlr_predicate]
fn multiple_if_else(c: &Ctx) {
    if c.x > 0 {
        c.y > 0
    } else {
        c.y < 0
    };
    if c.x < 100 {
        c.y < 100
    } else {
        c.y > 100
    };
}

#[cvlr_predicate]
fn if_else_with_let(c: &Ctx) {
    let threshold = 0;
    if c.x > threshold {
        c.y > threshold
    } else {
        c.y < threshold
    };
}

#[cvlr_predicate]
fn if_else_with_multiple_lets(c: &Ctx) {
    let min_val = 0;
    let max_val = 100;
    if c.x > min_val {
        c.y > min_val
    } else {
        c.y < min_val
    };
    if c.x < max_val {
        c.y < max_val
    } else {
        c.y > max_val
    };
}

#[test]
fn test_predicate() {
    let ctx = Ctx { x: 5, y: 50 };
    let pred = XGtZero;
    assert!(pred.eval(&ctx));

    let pred2 = YLtHundred;
    assert!(pred2.eval(&ctx));

    let pred3 = MultipleConditions;
    assert!(pred3.eval(&ctx));

    let pred4 = WithLetStatement;
    assert!(pred4.eval(&ctx));

    let pred5 = WithMultipleLets;
    assert!(pred5.eval(&ctx));

    let pred6 = LetBeforeExpressions;
    assert!(!pred6.eval(&ctx));

    let pred7 = WithIfElse;
    assert!(pred7.eval(&ctx));

    let pred8 = WithIfElseTrue;
    assert!(pred8.eval(&ctx));

    let pred9 = WithIfElseBothTrue;
    assert!(pred9.eval(&ctx));

    let pred10 = WithNestedIfElse;
    assert!(pred10.eval(&ctx));

    let pred11 = MultipleIfElse;
    assert!(pred11.eval(&ctx));

    let pred12 = IfElseWithLet;
    assert!(pred12.eval(&ctx));

    let pred13 = IfElseWithMultipleLets;
    assert!(pred13.eval(&ctx));
}

#[test]
fn test_if_else_predicates() {
    let ctx1 = Ctx { x: 5, y: 10 };
    let ctx2 = Ctx { x: -5, y: -10 };
    let ctx3 = Ctx { x: 5, y: -10 };
    let ctx4 = Ctx { x: -5, y: 10 };

    let pred = WithIfElse;
    assert!(pred.eval(&ctx1)); // x > 0 && y > 0
    assert!(pred.eval(&ctx2)); // x <= 0 && y < 0
    assert!(!pred.eval(&ctx3)); // x > 0 but y <= 0
    assert!(!pred.eval(&ctx4)); // x <= 0 but y >= 0

    let pred2 = WithIfElseTrue;
    assert!(pred2.eval(&ctx1)); // x > 0 && y > 0
    assert!(pred2.eval(&ctx2)); // x <= 0 && true
    assert!(!pred2.eval(&ctx3)); // x > 0 but y <= 0
    assert!(pred2.eval(&ctx4)); // x <= 0 && true

    let pred3 = WithIfElseBothTrue;
    assert!(pred3.eval(&ctx1)); // always true
    assert!(pred3.eval(&ctx2)); // always true
    assert!(pred3.eval(&ctx3)); // always true
    assert!(pred3.eval(&ctx4)); // always true

    let pred4 = WithNestedIfElse;
    assert!(pred4.eval(&ctx1)); // x > 0 && y > 0 && x + y > 0
    assert!(pred4.eval(&ctx2)); // x <= 0 && y < 0
    assert!(pred4.eval(&ctx3)); // x > 0 && y <= 0 && x > y
    assert!(!pred4.eval(&ctx4)); // x <= 0 && y >= 0
}
