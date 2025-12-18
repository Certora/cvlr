use cvlr_macros::cvlr_predicate;
use cvlr_spec::CvlrBoolExpr;
struct Ctx {
    x: i32,
    y: i32,
}
#[allow(unused_must_use, dead_code)]
pub fn x_gt_zero(c: &Ctx) {
    c.x > 0;
}
pub struct XGtZero;
impl ::cvlr::spec::CvlrBoolExpr for XGtZero {
    type Context = Ctx;
    fn eval(&self, ctx: &Self::Context) -> bool {
        let c = ctx;
        {
            if !(c.x > 0) {
                return false;
            }
            true
        }
    }
    fn assert(&self, ctx: &Self::Context) {
        let c = ctx;
        {
            let __cvlr_lhs = c.x;
            let __cvlr_rhs = 0;
            ::cvlr_log::cvlr_log("_", &("assert c.x > 0"));
            ::cvlr_log::cvlr_log("c.x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs > __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        };
    }
    fn assume(&self, ctx: &Self::Context) {
        let c = ctx;
        {
            let __cvlr_lhs = c.x;
            let __cvlr_rhs = 0;
            ::cvlr_log::cvlr_log("_", &("assume c.x > 0"));
            ::cvlr_log::cvlr_log("c.x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs > __cvlr_rhs);
        };
    }
}
#[allow(unused_must_use, dead_code)]
fn y_lt_hundred(c: &Ctx) {
    c.y < 100;
}
struct YLtHundred;
impl ::cvlr::spec::CvlrBoolExpr for YLtHundred {
    type Context = Ctx;
    fn eval(&self, ctx: &Self::Context) -> bool {
        let c = ctx;
        {
            if !(c.y < 100) {
                return false;
            }
            true
        }
    }
    fn assert(&self, ctx: &Self::Context) {
        let c = ctx;
        {
            let __cvlr_lhs = c.y;
            let __cvlr_rhs = 100;
            ::cvlr_log::cvlr_log("_", &("assert c.y < 100"));
            ::cvlr_log::cvlr_log("c.y", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("100", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs < __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        };
    }
    fn assume(&self, ctx: &Self::Context) {
        let c = ctx;
        {
            let __cvlr_lhs = c.y;
            let __cvlr_rhs = 100;
            ::cvlr_log::cvlr_log("_", &("assume c.y < 100"));
            ::cvlr_log::cvlr_log("c.y", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("100", &(__cvlr_rhs));
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
        };
    }
}
#[allow(unused_must_use, dead_code)]
fn multiple_conditions(c: &Ctx) {
    c.x > 0;
    c.y < 100;
}
struct MultipleConditions;
impl ::cvlr::spec::CvlrBoolExpr for MultipleConditions {
    type Context = Ctx;
    fn eval(&self, ctx: &Self::Context) -> bool {
        let c = ctx;
        {
            if !(c.x > 0) {
                return false;
            }
            if !(c.y < 100) {
                return false;
            }
            true
        }
    }
    fn assert(&self, ctx: &Self::Context) {
        let c = ctx;
        {
            let __cvlr_lhs = c.x;
            let __cvlr_rhs = 0;
            ::cvlr_log::cvlr_log("_", &("assert c.x > 0"));
            ::cvlr_log::cvlr_log("c.x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs > __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        };
        {
            let __cvlr_lhs = c.y;
            let __cvlr_rhs = 100;
            ::cvlr_log::cvlr_log("_", &("assert c.y < 100"));
            ::cvlr_log::cvlr_log("c.y", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("100", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs < __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        };
    }
    fn assume(&self, ctx: &Self::Context) {
        let c = ctx;
        {
            let __cvlr_lhs = c.x;
            let __cvlr_rhs = 0;
            ::cvlr_log::cvlr_log("_", &("assume c.x > 0"));
            ::cvlr_log::cvlr_log("c.x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs > __cvlr_rhs);
        };
        {
            let __cvlr_lhs = c.y;
            let __cvlr_rhs = 100;
            ::cvlr_log::cvlr_log("_", &("assume c.y < 100"));
            ::cvlr_log::cvlr_log("c.y", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("100", &(__cvlr_rhs));
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
        };
    }
}
#[allow(unused_must_use, dead_code)]
fn with_let_statement(c: &Ctx) {
    let threshold = 0;
    c.x > threshold;
}
struct WithLetStatement;
impl ::cvlr::spec::CvlrBoolExpr for WithLetStatement {
    type Context = Ctx;
    fn eval(&self, ctx: &Self::Context) -> bool {
        let c = ctx;
        {
            let threshold = 0;
            if !(c.x > threshold) {
                return false;
            }
            true
        }
    }
    fn assert(&self, ctx: &Self::Context) {
        let c = ctx;
        let threshold = 0;
        {
            let __cvlr_lhs = c.x;
            let __cvlr_rhs = threshold;
            ::cvlr_log::cvlr_log("_", &("assert c.x > threshold"));
            ::cvlr_log::cvlr_log("c.x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("threshold", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs > __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        };
    }
    fn assume(&self, ctx: &Self::Context) {
        let c = ctx;
        let threshold = 0;
        {
            let __cvlr_lhs = c.x;
            let __cvlr_rhs = threshold;
            ::cvlr_log::cvlr_log("_", &("assume c.x > threshold"));
            ::cvlr_log::cvlr_log("c.x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("threshold", &(__cvlr_rhs));
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs > __cvlr_rhs);
        };
    }
}
#[allow(unused_must_use, dead_code)]
fn with_multiple_lets(c: &Ctx) {
    let min_x = 0;
    let max_y = 100;
    c.x > min_x;
    c.y < max_y;
}
struct WithMultipleLets;
impl ::cvlr::spec::CvlrBoolExpr for WithMultipleLets {
    type Context = Ctx;
    fn eval(&self, ctx: &Self::Context) -> bool {
        let c = ctx;
        {
            let min_x = 0;
            let max_y = 100;
            if !(c.x > min_x) {
                return false;
            }
            if !(c.y < max_y) {
                return false;
            }
            true
        }
    }
    fn assert(&self, ctx: &Self::Context) {
        let c = ctx;
        let min_x = 0;
        let max_y = 100;
        {
            let __cvlr_lhs = c.x;
            let __cvlr_rhs = min_x;
            ::cvlr_log::cvlr_log("_", &("assert c.x > min_x"));
            ::cvlr_log::cvlr_log("c.x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("min_x", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs > __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        };
        {
            let __cvlr_lhs = c.y;
            let __cvlr_rhs = max_y;
            ::cvlr_log::cvlr_log("_", &("assert c.y < max_y"));
            ::cvlr_log::cvlr_log("c.y", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("max_y", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs < __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        };
    }
    fn assume(&self, ctx: &Self::Context) {
        let c = ctx;
        let min_x = 0;
        let max_y = 100;
        {
            let __cvlr_lhs = c.x;
            let __cvlr_rhs = min_x;
            ::cvlr_log::cvlr_log("_", &("assume c.x > min_x"));
            ::cvlr_log::cvlr_log("c.x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("min_x", &(__cvlr_rhs));
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs > __cvlr_rhs);
        };
        {
            let __cvlr_lhs = c.y;
            let __cvlr_rhs = max_y;
            ::cvlr_log::cvlr_log("_", &("assume c.y < max_y"));
            ::cvlr_log::cvlr_log("c.y", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("max_y", &(__cvlr_rhs));
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
        };
    }
}
#[allow(unused_must_use, dead_code)]
fn let_before_expressions(c: &Ctx) {
    let threshold = 5;
    let limit = 100;
    c.x > threshold;
    c.y < limit;
    c.x + c.y > threshold;
}
struct LetBeforeExpressions;
impl ::cvlr::spec::CvlrBoolExpr for LetBeforeExpressions {
    type Context = Ctx;
    fn eval(&self, ctx: &Self::Context) -> bool {
        let c = ctx;
        {
            let threshold = 5;
            let limit = 100;
            if !(c.x > threshold) {
                return false;
            }
            if !(c.y < limit) {
                return false;
            }
            if !(c.x + c.y > threshold) {
                return false;
            }
            true
        }
    }
    fn assert(&self, ctx: &Self::Context) {
        let c = ctx;
        let threshold = 5;
        let limit = 100;
        {
            let __cvlr_lhs = c.x;
            let __cvlr_rhs = threshold;
            ::cvlr_log::cvlr_log("_", &("assert c.x > threshold"));
            ::cvlr_log::cvlr_log("c.x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("threshold", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs > __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        };
        {
            let __cvlr_lhs = c.y;
            let __cvlr_rhs = limit;
            ::cvlr_log::cvlr_log("_", &("assert c.y < limit"));
            ::cvlr_log::cvlr_log("c.y", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("limit", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs < __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        };
        {
            let __cvlr_lhs = c.x + c.y;
            let __cvlr_rhs = threshold;
            ::cvlr_log::cvlr_log("_", &("assert c.x + c.y > threshold"));
            ::cvlr_log::cvlr_log("c.x + c.y", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("threshold", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs > __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        };
    }
    fn assume(&self, ctx: &Self::Context) {
        let c = ctx;
        let threshold = 5;
        let limit = 100;
        {
            let __cvlr_lhs = c.x;
            let __cvlr_rhs = threshold;
            ::cvlr_log::cvlr_log("_", &("assume c.x > threshold"));
            ::cvlr_log::cvlr_log("c.x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("threshold", &(__cvlr_rhs));
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs > __cvlr_rhs);
        };
        {
            let __cvlr_lhs = c.y;
            let __cvlr_rhs = limit;
            ::cvlr_log::cvlr_log("_", &("assume c.y < limit"));
            ::cvlr_log::cvlr_log("c.y", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("limit", &(__cvlr_rhs));
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
        };
        {
            let __cvlr_lhs = c.x + c.y;
            let __cvlr_rhs = threshold;
            ::cvlr_log::cvlr_log("_", &("assume c.x + c.y > threshold"));
            ::cvlr_log::cvlr_log("c.x + c.y", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("threshold", &(__cvlr_rhs));
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs > __cvlr_rhs);
        };
    }
}
