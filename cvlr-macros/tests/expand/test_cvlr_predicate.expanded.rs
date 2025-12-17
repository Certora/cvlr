use cvlr_macros::cvlr_predicate;
use cvlr_spec::CvlrBoolExpr;
struct Ctx {
    x: i32,
    y: i32,
}
pub fn x_gt_zero(c: &Ctx) {
    c.x > 0;
}
pub struct XGtZero;
impl ::cvlr_spec::CvlrBoolExpr<Ctx> for XGtZero {
    fn eval(&self, ctx: &Ctx) -> bool {
        let c = ctx;
        {
            let __cvlr_eval_all_res = true;
            let __cvlr_eval_all_res = __cvlr_eval_all_res && { c.x > 0 };
            __cvlr_eval_all_res
        }
    }
    fn assert(&self, ctx: &Ctx) {
        let c = ctx;
        {
            let lhs = c.x;
            let rhs = 0;
            ::cvlr_log::cvlr_log("_", &("assert c.x > 0"));
            ::cvlr_log::cvlr_log("c.x", &(lhs));
            ::cvlr_log::cvlr_log("0", &(rhs));
            {
                let c_ = lhs > rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        };
    }
    fn assume(&self, ctx: &Ctx) {
        let c = ctx;
        {
            let lhs = c.x;
            let rhs = 0;
            ::cvlr_log::cvlr_log("_", &("assume c.x > 0"));
            ::cvlr_log::cvlr_log("c.x", &(lhs));
            ::cvlr_log::cvlr_log("0", &(rhs));
            ::cvlr_asserts::cvlr_assume_checked(lhs > rhs);
        };
    }
}
fn y_lt_hundred(c: &Ctx) {
    c.y < 100;
}
struct YLtHundred;
impl ::cvlr_spec::CvlrBoolExpr<Ctx> for YLtHundred {
    fn eval(&self, ctx: &Ctx) -> bool {
        let c = ctx;
        {
            let __cvlr_eval_all_res = true;
            let __cvlr_eval_all_res = __cvlr_eval_all_res && { c.y < 100 };
            __cvlr_eval_all_res
        }
    }
    fn assert(&self, ctx: &Ctx) {
        let c = ctx;
        {
            let lhs = c.y;
            let rhs = 100;
            ::cvlr_log::cvlr_log("_", &("assert c.y < 100"));
            ::cvlr_log::cvlr_log("c.y", &(lhs));
            ::cvlr_log::cvlr_log("100", &(rhs));
            {
                let c_ = lhs < rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        };
    }
    fn assume(&self, ctx: &Ctx) {
        let c = ctx;
        {
            let lhs = c.y;
            let rhs = 100;
            ::cvlr_log::cvlr_log("_", &("assume c.y < 100"));
            ::cvlr_log::cvlr_log("c.y", &(lhs));
            ::cvlr_log::cvlr_log("100", &(rhs));
            ::cvlr_asserts::cvlr_assume_checked(lhs < rhs);
        };
    }
}
fn multiple_conditions(c: &Ctx) {
    c.x > 0;
    c.y < 100;
}
struct MultipleConditions;
impl ::cvlr_spec::CvlrBoolExpr<Ctx> for MultipleConditions {
    fn eval(&self, ctx: &Ctx) -> bool {
        let c = ctx;
        {
            let __cvlr_eval_all_res = true;
            let __cvlr_eval_all_res = __cvlr_eval_all_res && { c.x > 0 };
            let __cvlr_eval_all_res = __cvlr_eval_all_res && { c.y < 100 };
            __cvlr_eval_all_res
        }
    }
    fn assert(&self, ctx: &Ctx) {
        let c = ctx;
        {
            let lhs = c.x;
            let rhs = 0;
            ::cvlr_log::cvlr_log("_", &("assert c.x > 0"));
            ::cvlr_log::cvlr_log("c.x", &(lhs));
            ::cvlr_log::cvlr_log("0", &(rhs));
            {
                let c_ = lhs > rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        };
        {
            let lhs = c.y;
            let rhs = 100;
            ::cvlr_log::cvlr_log("_", &("assert c.y < 100"));
            ::cvlr_log::cvlr_log("c.y", &(lhs));
            ::cvlr_log::cvlr_log("100", &(rhs));
            {
                let c_ = lhs < rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        };
    }
    fn assume(&self, ctx: &Ctx) {
        let c = ctx;
        {
            let lhs = c.x;
            let rhs = 0;
            ::cvlr_log::cvlr_log("_", &("assume c.x > 0"));
            ::cvlr_log::cvlr_log("c.x", &(lhs));
            ::cvlr_log::cvlr_log("0", &(rhs));
            ::cvlr_asserts::cvlr_assume_checked(lhs > rhs);
        };
        {
            let lhs = c.y;
            let rhs = 100;
            ::cvlr_log::cvlr_log("_", &("assume c.y < 100"));
            ::cvlr_log::cvlr_log("c.y", &(lhs));
            ::cvlr_log::cvlr_log("100", &(rhs));
            ::cvlr_asserts::cvlr_assume_checked(lhs < rhs);
        };
    }
}
fn with_let_statement(c: &Ctx) {
    let threshold = 0;
    c.x > threshold;
}
struct WithLetStatement;
impl ::cvlr_spec::CvlrBoolExpr<Ctx> for WithLetStatement {
    fn eval(&self, ctx: &Ctx) -> bool {
        let c = ctx;
        {
            let threshold = 0;
            let __cvlr_eval_all_res = true;
            let __cvlr_eval_all_res = __cvlr_eval_all_res && { c.x > threshold };
            __cvlr_eval_all_res
        }
    }
    fn assert(&self, ctx: &Ctx) {
        let c = ctx;
        let threshold = 0;
        {
            let lhs = c.x;
            let rhs = threshold;
            ::cvlr_log::cvlr_log("_", &("assert c.x > threshold"));
            ::cvlr_log::cvlr_log("c.x", &(lhs));
            ::cvlr_log::cvlr_log("threshold", &(rhs));
            {
                let c_ = lhs > rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        };
    }
    fn assume(&self, ctx: &Ctx) {
        let c = ctx;
        let threshold = 0;
        {
            let lhs = c.x;
            let rhs = threshold;
            ::cvlr_log::cvlr_log("_", &("assume c.x > threshold"));
            ::cvlr_log::cvlr_log("c.x", &(lhs));
            ::cvlr_log::cvlr_log("threshold", &(rhs));
            ::cvlr_asserts::cvlr_assume_checked(lhs > rhs);
        };
    }
}
fn with_multiple_lets(c: &Ctx) {
    let min_x = 0;
    let max_y = 100;
    c.x > min_x;
    c.y < max_y;
}
struct WithMultipleLets;
impl ::cvlr_spec::CvlrBoolExpr<Ctx> for WithMultipleLets {
    fn eval(&self, ctx: &Ctx) -> bool {
        let c = ctx;
        {
            let min_x = 0;
            let max_y = 100;
            let __cvlr_eval_all_res = true;
            let __cvlr_eval_all_res = __cvlr_eval_all_res && { c.x > min_x };
            let __cvlr_eval_all_res = __cvlr_eval_all_res && { c.y < max_y };
            __cvlr_eval_all_res
        }
    }
    fn assert(&self, ctx: &Ctx) {
        let c = ctx;
        let min_x = 0;
        let max_y = 100;
        {
            let lhs = c.x;
            let rhs = min_x;
            ::cvlr_log::cvlr_log("_", &("assert c.x > min_x"));
            ::cvlr_log::cvlr_log("c.x", &(lhs));
            ::cvlr_log::cvlr_log("min_x", &(rhs));
            {
                let c_ = lhs > rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        };
        {
            let lhs = c.y;
            let rhs = max_y;
            ::cvlr_log::cvlr_log("_", &("assert c.y < max_y"));
            ::cvlr_log::cvlr_log("c.y", &(lhs));
            ::cvlr_log::cvlr_log("max_y", &(rhs));
            {
                let c_ = lhs < rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        };
    }
    fn assume(&self, ctx: &Ctx) {
        let c = ctx;
        let min_x = 0;
        let max_y = 100;
        {
            let lhs = c.x;
            let rhs = min_x;
            ::cvlr_log::cvlr_log("_", &("assume c.x > min_x"));
            ::cvlr_log::cvlr_log("c.x", &(lhs));
            ::cvlr_log::cvlr_log("min_x", &(rhs));
            ::cvlr_asserts::cvlr_assume_checked(lhs > rhs);
        };
        {
            let lhs = c.y;
            let rhs = max_y;
            ::cvlr_log::cvlr_log("_", &("assume c.y < max_y"));
            ::cvlr_log::cvlr_log("c.y", &(lhs));
            ::cvlr_log::cvlr_log("max_y", &(rhs));
            ::cvlr_asserts::cvlr_assume_checked(lhs < rhs);
        };
    }
}
fn let_before_expressions(c: &Ctx) {
    let threshold = 5;
    let limit = 100;
    c.x > threshold;
    c.y < limit;
    c.x + c.y > threshold;
}
struct LetBeforeExpressions;
impl ::cvlr_spec::CvlrBoolExpr<Ctx> for LetBeforeExpressions {
    fn eval(&self, ctx: &Ctx) -> bool {
        let c = ctx;
        {
            let threshold = 5;
            let limit = 100;
            let __cvlr_eval_all_res = true;
            let __cvlr_eval_all_res = __cvlr_eval_all_res && { c.x > threshold };
            let __cvlr_eval_all_res = __cvlr_eval_all_res && { c.y < limit };
            let __cvlr_eval_all_res = __cvlr_eval_all_res && { c.x + c.y > threshold };
            __cvlr_eval_all_res
        }
    }
    fn assert(&self, ctx: &Ctx) {
        let c = ctx;
        let threshold = 5;
        let limit = 100;
        {
            let lhs = c.x;
            let rhs = threshold;
            ::cvlr_log::cvlr_log("_", &("assert c.x > threshold"));
            ::cvlr_log::cvlr_log("c.x", &(lhs));
            ::cvlr_log::cvlr_log("threshold", &(rhs));
            {
                let c_ = lhs > rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        };
        {
            let lhs = c.y;
            let rhs = limit;
            ::cvlr_log::cvlr_log("_", &("assert c.y < limit"));
            ::cvlr_log::cvlr_log("c.y", &(lhs));
            ::cvlr_log::cvlr_log("limit", &(rhs));
            {
                let c_ = lhs < rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        };
        {
            let lhs = c.x + c.y;
            let rhs = threshold;
            ::cvlr_log::cvlr_log("_", &("assert c.x + c.y > threshold"));
            ::cvlr_log::cvlr_log("c.x + c.y", &(lhs));
            ::cvlr_log::cvlr_log("threshold", &(rhs));
            {
                let c_ = lhs > rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        };
    }
    fn assume(&self, ctx: &Ctx) {
        let c = ctx;
        let threshold = 5;
        let limit = 100;
        {
            let lhs = c.x;
            let rhs = threshold;
            ::cvlr_log::cvlr_log("_", &("assume c.x > threshold"));
            ::cvlr_log::cvlr_log("c.x", &(lhs));
            ::cvlr_log::cvlr_log("threshold", &(rhs));
            ::cvlr_asserts::cvlr_assume_checked(lhs > rhs);
        };
        {
            let lhs = c.y;
            let rhs = limit;
            ::cvlr_log::cvlr_log("_", &("assume c.y < limit"));
            ::cvlr_log::cvlr_log("c.y", &(lhs));
            ::cvlr_log::cvlr_log("limit", &(rhs));
            ::cvlr_asserts::cvlr_assume_checked(lhs < rhs);
        };
        {
            let lhs = c.x + c.y;
            let rhs = threshold;
            ::cvlr_log::cvlr_log("_", &("assume c.x + c.y > threshold"));
            ::cvlr_log::cvlr_log("c.x + c.y", &(lhs));
            ::cvlr_log::cvlr_log("threshold", &(rhs));
            ::cvlr_asserts::cvlr_assume_checked(lhs > rhs);
        };
    }
}
