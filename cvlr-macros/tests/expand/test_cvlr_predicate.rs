use cvlr_macros::cvlr_predicate;
use cvlr_spec::CvlrBoolExpr;

struct Ctx {
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
    assert!(pred6.eval(&ctx));
}

