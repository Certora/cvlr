use cvlr_asserts::{cvlr_assert_le, cvlr_assert_lt, cvlr_assert_ge, cvlr_assert_gt};
fn main() {
    {
        let lhs = 1;
        let rhs = 2;
        ::cvlr_log::cvlr_log("_", &("assert 1 <= 2"));
        ::cvlr_log::cvlr_log("1", &(lhs));
        ::cvlr_log::cvlr_log("2", &(rhs));
        {
            let c_ = lhs <= rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let lhs = 1;
        let rhs = 2;
        ::cvlr_log::cvlr_log("_", &("assert 1 < 2"));
        ::cvlr_log::cvlr_log("1", &(lhs));
        ::cvlr_log::cvlr_log("2", &(rhs));
        {
            let c_ = lhs < rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let lhs = 2;
        let rhs = 1;
        ::cvlr_log::cvlr_log("_", &("assert 2 >= 1"));
        ::cvlr_log::cvlr_log("2", &(lhs));
        ::cvlr_log::cvlr_log("1", &(rhs));
        {
            let c_ = lhs >= rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let lhs = 2;
        let rhs = 1;
        ::cvlr_log::cvlr_log("_", &("assert 2 > 1"));
        ::cvlr_log::cvlr_log("2", &(lhs));
        ::cvlr_log::cvlr_log("1", &(rhs));
        {
            let c_ = lhs > rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let lhs = x;
        let rhs = y;
        ::cvlr_log::cvlr_log("_", &("assert x <= y"));
        ::cvlr_log::cvlr_log("x", &(lhs));
        ::cvlr_log::cvlr_log("y", &(rhs));
        {
            let c_ = lhs <= rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
}
