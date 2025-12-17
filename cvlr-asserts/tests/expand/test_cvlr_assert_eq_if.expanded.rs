use cvlr_asserts::cvlr_assert_eq_if;
fn main() {
    let x = 1;
    let flag = true;
    let a = 1;
    let b = 2;
    let x = 2;
    let y = 2;
    {
        let __cvlr_guard = x > 0;
        ::cvlr_log::cvlr_log("_", &("assert if x > 0 { a == b }"));
        ::cvlr_log::cvlr_log("x > 0", &(__cvlr_guard));
        if __cvlr_guard {
            let __cvlr_lhs = a;
            let __cvlr_rhs = b;
            ::cvlr_log::cvlr_log("a", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("b", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs == __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    {
        let __cvlr_guard = flag;
        ::cvlr_log::cvlr_log("_", &("assert if flag { x == y }"));
        ::cvlr_log::cvlr_log("flag", &(__cvlr_guard));
        if __cvlr_guard {
            let __cvlr_lhs = x;
            let __cvlr_rhs = y;
            ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("y", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs == __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
}
