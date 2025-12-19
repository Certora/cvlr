use cvlr_macros::cvlr_predicate;
pub struct Ctx {
    x: i32,
    y: i32,
}
#[allow(unused_must_use, dead_code)]
pub fn x_gt_zero(c: &Ctx) {
    c.x > 0;
}
pub struct XGtZero;
impl ::cvlr::spec::CvlrFormula for XGtZero {
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
            cvlr::log::log_scope_start("assert");
            ::cvlr_log::cvlr_log("_", &("c.x > 0"));
            ::cvlr_log::cvlr_log("c.x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
            cvlr::log::log_scope_start("assert");
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
            cvlr::log::log_scope_start("assume");
            ::cvlr_log::cvlr_log("_", &("c.x > 0"));
            ::cvlr_log::cvlr_log("c.x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
            cvlr::log::log_scope_end("assume");
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs > __cvlr_rhs);
        };
    }
}
impl ::cvlr::spec::CvlrPredicate for XGtZero {}
#[allow(unused_must_use, dead_code)]
fn y_lt_hundred(c: &Ctx) {
    c.y < 100;
}
struct YLtHundred;
impl ::cvlr::spec::CvlrFormula for YLtHundred {
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
            cvlr::log::log_scope_start("assert");
            ::cvlr_log::cvlr_log("_", &("c.y < 100"));
            ::cvlr_log::cvlr_log("c.y", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("100", &(__cvlr_rhs));
            cvlr::log::log_scope_start("assert");
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
            cvlr::log::log_scope_start("assume");
            ::cvlr_log::cvlr_log("_", &("c.y < 100"));
            ::cvlr_log::cvlr_log("c.y", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("100", &(__cvlr_rhs));
            cvlr::log::log_scope_end("assume");
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
        };
    }
}
impl ::cvlr::spec::CvlrPredicate for YLtHundred {}
#[allow(unused_must_use, dead_code)]
fn multiple_conditions(c: &Ctx) {
    c.x > 0;
    c.y < 100;
}
struct MultipleConditions;
impl ::cvlr::spec::CvlrFormula for MultipleConditions {
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
            cvlr::log::log_scope_start("assert");
            ::cvlr_log::cvlr_log("_", &("c.x > 0"));
            ::cvlr_log::cvlr_log("c.x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
            cvlr::log::log_scope_start("assert");
            {
                let c_ = __cvlr_lhs > __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        };
        {
            let __cvlr_lhs = c.y;
            let __cvlr_rhs = 100;
            cvlr::log::log_scope_start("assert");
            ::cvlr_log::cvlr_log("_", &("c.y < 100"));
            ::cvlr_log::cvlr_log("c.y", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("100", &(__cvlr_rhs));
            cvlr::log::log_scope_start("assert");
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
            cvlr::log::log_scope_start("assume");
            ::cvlr_log::cvlr_log("_", &("c.x > 0"));
            ::cvlr_log::cvlr_log("c.x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
            cvlr::log::log_scope_end("assume");
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs > __cvlr_rhs);
        };
        {
            let __cvlr_lhs = c.y;
            let __cvlr_rhs = 100;
            cvlr::log::log_scope_start("assume");
            ::cvlr_log::cvlr_log("_", &("c.y < 100"));
            ::cvlr_log::cvlr_log("c.y", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("100", &(__cvlr_rhs));
            cvlr::log::log_scope_end("assume");
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
        };
    }
}
impl ::cvlr::spec::CvlrPredicate for MultipleConditions {}
#[allow(unused_must_use, dead_code)]
fn with_let_statement(c: &Ctx) {
    let threshold = 0;
    c.x > threshold;
}
struct WithLetStatement;
impl ::cvlr::spec::CvlrFormula for WithLetStatement {
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
            cvlr::log::log_scope_start("assert");
            ::cvlr_log::cvlr_log("_", &("c.x > threshold"));
            ::cvlr_log::cvlr_log("c.x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("threshold", &(__cvlr_rhs));
            cvlr::log::log_scope_start("assert");
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
            cvlr::log::log_scope_start("assume");
            ::cvlr_log::cvlr_log("_", &("c.x > threshold"));
            ::cvlr_log::cvlr_log("c.x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("threshold", &(__cvlr_rhs));
            cvlr::log::log_scope_end("assume");
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs > __cvlr_rhs);
        };
    }
}
impl ::cvlr::spec::CvlrPredicate for WithLetStatement {}
#[allow(unused_must_use, dead_code)]
fn with_multiple_lets(c: &Ctx) {
    let min_x = 0;
    let max_y = 100;
    c.x > min_x;
    c.y < max_y;
}
struct WithMultipleLets;
impl ::cvlr::spec::CvlrFormula for WithMultipleLets {
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
            cvlr::log::log_scope_start("assert");
            ::cvlr_log::cvlr_log("_", &("c.x > min_x"));
            ::cvlr_log::cvlr_log("c.x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("min_x", &(__cvlr_rhs));
            cvlr::log::log_scope_start("assert");
            {
                let c_ = __cvlr_lhs > __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        };
        {
            let __cvlr_lhs = c.y;
            let __cvlr_rhs = max_y;
            cvlr::log::log_scope_start("assert");
            ::cvlr_log::cvlr_log("_", &("c.y < max_y"));
            ::cvlr_log::cvlr_log("c.y", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("max_y", &(__cvlr_rhs));
            cvlr::log::log_scope_start("assert");
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
            cvlr::log::log_scope_start("assume");
            ::cvlr_log::cvlr_log("_", &("c.x > min_x"));
            ::cvlr_log::cvlr_log("c.x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("min_x", &(__cvlr_rhs));
            cvlr::log::log_scope_end("assume");
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs > __cvlr_rhs);
        };
        {
            let __cvlr_lhs = c.y;
            let __cvlr_rhs = max_y;
            cvlr::log::log_scope_start("assume");
            ::cvlr_log::cvlr_log("_", &("c.y < max_y"));
            ::cvlr_log::cvlr_log("c.y", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("max_y", &(__cvlr_rhs));
            cvlr::log::log_scope_end("assume");
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
        };
    }
}
impl ::cvlr::spec::CvlrPredicate for WithMultipleLets {}
#[allow(unused_must_use, dead_code)]
fn let_before_expressions(c: &Ctx) {
    let threshold = 5;
    let limit = 100;
    c.x > threshold;
    c.y < limit;
    c.x + c.y > threshold;
}
struct LetBeforeExpressions;
impl ::cvlr::spec::CvlrFormula for LetBeforeExpressions {
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
            cvlr::log::log_scope_start("assert");
            ::cvlr_log::cvlr_log("_", &("c.x > threshold"));
            ::cvlr_log::cvlr_log("c.x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("threshold", &(__cvlr_rhs));
            cvlr::log::log_scope_start("assert");
            {
                let c_ = __cvlr_lhs > __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        };
        {
            let __cvlr_lhs = c.y;
            let __cvlr_rhs = limit;
            cvlr::log::log_scope_start("assert");
            ::cvlr_log::cvlr_log("_", &("c.y < limit"));
            ::cvlr_log::cvlr_log("c.y", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("limit", &(__cvlr_rhs));
            cvlr::log::log_scope_start("assert");
            {
                let c_ = __cvlr_lhs < __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        };
        {
            let __cvlr_lhs = c.x + c.y;
            let __cvlr_rhs = threshold;
            cvlr::log::log_scope_start("assert");
            ::cvlr_log::cvlr_log("_", &("c.x + c.y > threshold"));
            ::cvlr_log::cvlr_log("c.x + c.y", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("threshold", &(__cvlr_rhs));
            cvlr::log::log_scope_start("assert");
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
            cvlr::log::log_scope_start("assume");
            ::cvlr_log::cvlr_log("_", &("c.x > threshold"));
            ::cvlr_log::cvlr_log("c.x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("threshold", &(__cvlr_rhs));
            cvlr::log::log_scope_end("assume");
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs > __cvlr_rhs);
        };
        {
            let __cvlr_lhs = c.y;
            let __cvlr_rhs = limit;
            cvlr::log::log_scope_start("assume");
            ::cvlr_log::cvlr_log("_", &("c.y < limit"));
            ::cvlr_log::cvlr_log("c.y", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("limit", &(__cvlr_rhs));
            cvlr::log::log_scope_end("assume");
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
        };
        {
            let __cvlr_lhs = c.x + c.y;
            let __cvlr_rhs = threshold;
            cvlr::log::log_scope_start("assume");
            ::cvlr_log::cvlr_log("_", &("c.x + c.y > threshold"));
            ::cvlr_log::cvlr_log("c.x + c.y", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("threshold", &(__cvlr_rhs));
            cvlr::log::log_scope_end("assume");
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs > __cvlr_rhs);
        };
    }
}
impl ::cvlr::spec::CvlrPredicate for LetBeforeExpressions {}
#[allow(unused_must_use, dead_code)]
fn with_if_else(c: &Ctx) {
    if c.x > 0 { c.y > 0 } else { c.y < 0 };
}
struct WithIfElse;
impl ::cvlr::spec::CvlrFormula for WithIfElse {
    type Context = Ctx;
    fn eval(&self, ctx: &Self::Context) -> bool {
        let c = ctx;
        {
            if !(if c.x > 0 { c.y > 0 } else { c.y < 0 }) {
                return false;
            }
            true
        }
    }
    fn assert(&self, ctx: &Self::Context) {
        let c = ctx;
        if c.x > 0 {
            {
                let c_ = c.y > 0;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        } else {
            {
                let c_ = c.y < 0;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    }
    fn assume(&self, ctx: &Self::Context) {
        let c = ctx;
        if c.x > 0 {
            ::cvlr_asserts::cvlr_assume_checked(c.y > 0);
        } else {
            ::cvlr_asserts::cvlr_assume_checked(c.y < 0);
        }
    }
}
impl ::cvlr::spec::CvlrPredicate for WithIfElse {}
#[allow(unused_must_use, dead_code)]
fn with_if_else_true(c: &Ctx) {
    if c.x > 0 { c.y > 0 } else { true };
}
struct WithIfElseTrue;
impl ::cvlr::spec::CvlrFormula for WithIfElseTrue {
    type Context = Ctx;
    fn eval(&self, ctx: &Self::Context) -> bool {
        let c = ctx;
        {
            if !(if c.x > 0 { c.y > 0 } else { true }) {
                return false;
            }
            true
        }
    }
    fn assert(&self, ctx: &Self::Context) {
        let c = ctx;
        if c.x > 0 {
            {
                let c_ = c.y > 0;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        } else {
            ()
        }
    }
    fn assume(&self, ctx: &Self::Context) {
        let c = ctx;
        if c.x > 0 {
            ::cvlr_asserts::cvlr_assume_checked(c.y > 0);
        } else {
            ()
        }
    }
}
impl ::cvlr::spec::CvlrPredicate for WithIfElseTrue {}
#[allow(unused_must_use, dead_code)]
fn with_if_else_both_true(c: &Ctx) {
    if c.x > 0 { true } else { true };
}
struct WithIfElseBothTrue;
impl ::cvlr::spec::CvlrFormula for WithIfElseBothTrue {
    type Context = Ctx;
    fn eval(&self, ctx: &Self::Context) -> bool {
        let c = ctx;
        {
            if !(if c.x > 0 { true } else { true }) {
                return false;
            }
            true
        }
    }
    fn assert(&self, ctx: &Self::Context) {
        let c = ctx;
        if c.x > 0 { () } else { () }
    }
    fn assume(&self, ctx: &Self::Context) {
        let c = ctx;
        if c.x > 0 { () } else { () }
    }
}
impl ::cvlr::spec::CvlrPredicate for WithIfElseBothTrue {}
#[allow(unused_must_use, dead_code)]
fn with_nested_if_else(c: &Ctx) {
    if c.x > 0 { if c.y > 0 { c.x + c.y > 0 } else { c.x > c.y } } else { c.y < 0 };
}
struct WithNestedIfElse;
impl ::cvlr::spec::CvlrFormula for WithNestedIfElse {
    type Context = Ctx;
    fn eval(&self, ctx: &Self::Context) -> bool {
        let c = ctx;
        {
            if !(if c.x > 0 {
                if c.y > 0 { c.x + c.y > 0 } else { c.x > c.y }
            } else {
                c.y < 0
            }) {
                return false;
            }
            true
        }
    }
    fn assert(&self, ctx: &Self::Context) {
        let c = ctx;
        if c.x > 0 {
            if c.y > 0 {
                {
                    let c_ = c.x + c.y > 0;
                    ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                    ::cvlr_asserts::cvlr_assert_checked(c_);
                };
            } else {
                {
                    let c_ = c.x > c.y;
                    ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                    ::cvlr_asserts::cvlr_assert_checked(c_);
                };
            }
        } else {
            {
                let c_ = c.y < 0;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    }
    fn assume(&self, ctx: &Self::Context) {
        let c = ctx;
        if c.x > 0 {
            if c.y > 0 {
                ::cvlr_asserts::cvlr_assume_checked(c.x + c.y > 0);
            } else {
                ::cvlr_asserts::cvlr_assume_checked(c.x > c.y);
            }
        } else {
            ::cvlr_asserts::cvlr_assume_checked(c.y < 0);
        }
    }
}
impl ::cvlr::spec::CvlrPredicate for WithNestedIfElse {}
#[allow(unused_must_use, dead_code)]
fn multiple_if_else(c: &Ctx) {
    if c.x > 0 { c.y > 0 } else { c.y < 0 };
    if c.x < 100 { c.y < 100 } else { c.y > 100 };
}
struct MultipleIfElse;
impl ::cvlr::spec::CvlrFormula for MultipleIfElse {
    type Context = Ctx;
    fn eval(&self, ctx: &Self::Context) -> bool {
        let c = ctx;
        {
            if !(if c.x > 0 { c.y > 0 } else { c.y < 0 }) {
                return false;
            }
            if !(if c.x < 100 { c.y < 100 } else { c.y > 100 }) {
                return false;
            }
            true
        }
    }
    fn assert(&self, ctx: &Self::Context) {
        let c = ctx;
        if c.x > 0 {
            {
                let c_ = c.y > 0;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        } else {
            {
                let c_ = c.y < 0;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
        if c.x < 100 {
            {
                let c_ = c.y < 100;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        } else {
            {
                let c_ = c.y > 100;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    }
    fn assume(&self, ctx: &Self::Context) {
        let c = ctx;
        if c.x > 0 {
            ::cvlr_asserts::cvlr_assume_checked(c.y > 0);
        } else {
            ::cvlr_asserts::cvlr_assume_checked(c.y < 0);
        }
        if c.x < 100 {
            ::cvlr_asserts::cvlr_assume_checked(c.y < 100);
        } else {
            ::cvlr_asserts::cvlr_assume_checked(c.y > 100);
        }
    }
}
impl ::cvlr::spec::CvlrPredicate for MultipleIfElse {}
#[allow(unused_must_use, dead_code)]
fn if_else_with_let(c: &Ctx) {
    let threshold = 0;
    if c.x > threshold { c.y > threshold } else { c.y < threshold };
}
struct IfElseWithLet;
impl ::cvlr::spec::CvlrFormula for IfElseWithLet {
    type Context = Ctx;
    fn eval(&self, ctx: &Self::Context) -> bool {
        let c = ctx;
        {
            let threshold = 0;
            if !(if c.x > threshold { c.y > threshold } else { c.y < threshold }) {
                return false;
            }
            true
        }
    }
    fn assert(&self, ctx: &Self::Context) {
        let c = ctx;
        let threshold = 0;
        if c.x > threshold {
            {
                let c_ = c.y > threshold;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        } else {
            {
                let c_ = c.y < threshold;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    }
    fn assume(&self, ctx: &Self::Context) {
        let c = ctx;
        let threshold = 0;
        if c.x > threshold {
            ::cvlr_asserts::cvlr_assume_checked(c.y > threshold);
        } else {
            ::cvlr_asserts::cvlr_assume_checked(c.y < threshold);
        }
    }
}
impl ::cvlr::spec::CvlrPredicate for IfElseWithLet {}
#[allow(unused_must_use, dead_code)]
fn if_else_with_multiple_lets(c: &Ctx) {
    let min_val = 0;
    let max_val = 100;
    if c.x > min_val { c.y > min_val } else { c.y < min_val };
    if c.x < max_val { c.y < max_val } else { c.y > max_val };
}
struct IfElseWithMultipleLets;
impl ::cvlr::spec::CvlrFormula for IfElseWithMultipleLets {
    type Context = Ctx;
    fn eval(&self, ctx: &Self::Context) -> bool {
        let c = ctx;
        {
            let min_val = 0;
            let max_val = 100;
            if !(if c.x > min_val { c.y > min_val } else { c.y < min_val }) {
                return false;
            }
            if !(if c.x < max_val { c.y < max_val } else { c.y > max_val }) {
                return false;
            }
            true
        }
    }
    fn assert(&self, ctx: &Self::Context) {
        let c = ctx;
        let min_val = 0;
        let max_val = 100;
        if c.x > min_val {
            {
                let c_ = c.y > min_val;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        } else {
            {
                let c_ = c.y < min_val;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
        if c.x < max_val {
            {
                let c_ = c.y < max_val;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        } else {
            {
                let c_ = c.y > max_val;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    }
    fn assume(&self, ctx: &Self::Context) {
        let c = ctx;
        let min_val = 0;
        let max_val = 100;
        if c.x > min_val {
            ::cvlr_asserts::cvlr_assume_checked(c.y > min_val);
        } else {
            ::cvlr_asserts::cvlr_assume_checked(c.y < min_val);
        }
        if c.x < max_val {
            ::cvlr_asserts::cvlr_assume_checked(c.y < max_val);
        } else {
            ::cvlr_asserts::cvlr_assume_checked(c.y > max_val);
        }
    }
}
impl ::cvlr::spec::CvlrPredicate for IfElseWithMultipleLets {}
pub fn main() {}
