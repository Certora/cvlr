use cvlr_spec::cvlr_predicate;
struct Ctx {
    x: i32,
}
fn main() {
    let _ = {
        #[allow(unused_must_use, dead_code)]
        fn __anonymous_predicate(c: &Ctx) {
            let threshold = 0;
            c.x > threshold;
        }
        struct AnonymousPredicate;
        impl ::cvlr::spec::CvlrFormula for AnonymousPredicate {
            type Context = Ctx;
            fn eval(&self, ctx: &Self::Context) -> bool {
                let c = ctx;
                {
                    let threshold = 0;
                    let mut __cvlr_eval_res = true;
                    __cvlr_eval_res = __cvlr_eval_res && { c.x > threshold };
                    __cvlr_eval_res
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
                    cvlr::log::log_scope_end("assert");
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
        impl ::cvlr::spec::CvlrPredicate for AnonymousPredicate {}
        AnonymousPredicate
    };
}
