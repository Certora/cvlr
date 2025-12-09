use cvlr_asserts::cvlr_assert_eq;
fn main() {
    {
        let lhs = 1;
        let rhs = 1;
        ::cvlr_log::cvlr_log("stringify! (assert 1 == 1)", &("assert 1 == 1"));
        ::cvlr_log::cvlr_log("1", &(lhs));
        ::cvlr_log::cvlr_log("1", &(rhs));
        {
            let c_ = lhs == rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let lhs = x;
        let rhs = y;
        ::cvlr_log::cvlr_log("stringify! (assert x == y)", &("assert x == y"));
        ::cvlr_log::cvlr_log("x", &(lhs));
        ::cvlr_log::cvlr_log("y", &(rhs));
        {
            let c_ = lhs == rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let lhs = a;
        let rhs = b;
        ::cvlr_log::cvlr_log("stringify! (assert a == b)", &("assert a == b"));
        ::cvlr_log::cvlr_log("a", &(lhs));
        ::cvlr_log::cvlr_log("b", &(rhs));
        {
            let c_ = lhs == rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
}
