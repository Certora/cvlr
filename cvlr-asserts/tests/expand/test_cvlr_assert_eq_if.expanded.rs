use cvlr_asserts::cvlr_assert_eq_if;
fn main() {
    {
        let guard = x > 0;
        ::cvlr_log::cvlr_log(
            "stringify! (assert x > 0 == > a == b)",
            &("assert x > 0 == > a == b"),
        );
        ::cvlr_log::cvlr_log("x > 0", &(guard));
        if guard {
            let lhs = a;
            let rhs = b;
            ::cvlr_log::cvlr_log("a", &(lhs));
            ::cvlr_log::cvlr_log("b", &(rhs));
            {
                let c_ = lhs == rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    {
        let guard = flag;
        ::cvlr_log::cvlr_log(
            "stringify! (assert flag == > x == y)",
            &("assert flag == > x == y"),
        );
        ::cvlr_log::cvlr_log("flag", &(guard));
        if guard {
            let lhs = x;
            let rhs = y;
            ::cvlr_log::cvlr_log("x", &(lhs));
            ::cvlr_log::cvlr_log("y", &(rhs));
            {
                let c_ = lhs == rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
}
