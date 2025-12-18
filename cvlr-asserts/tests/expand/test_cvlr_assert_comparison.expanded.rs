use cvlr_asserts::{cvlr_assert_le, cvlr_assert_lt, cvlr_assert_ge, cvlr_assert_gt};
fn main() {
    {
        let __cvlr_lhs = 1;
        let __cvlr_rhs = 2;
        cvlr::log::log_scope_start("assert");
        ::cvlr_log::cvlr_log("_", &("1 <= 2"));
        ::cvlr_log::cvlr_log("1", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("2", &(__cvlr_rhs));
        cvlr::log::log_scope_start("assert");
        {
            let c_ = __cvlr_lhs <= __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = 1;
        let __cvlr_rhs = 2;
        cvlr::log::log_scope_start("assert");
        ::cvlr_log::cvlr_log("_", &("1 < 2"));
        ::cvlr_log::cvlr_log("1", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("2", &(__cvlr_rhs));
        cvlr::log::log_scope_start("assert");
        {
            let c_ = __cvlr_lhs < __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = 2;
        let __cvlr_rhs = 1;
        cvlr::log::log_scope_start("assert");
        ::cvlr_log::cvlr_log("_", &("2 >= 1"));
        ::cvlr_log::cvlr_log("2", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("1", &(__cvlr_rhs));
        cvlr::log::log_scope_start("assert");
        {
            let c_ = __cvlr_lhs >= __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = 2;
        let __cvlr_rhs = 1;
        cvlr::log::log_scope_start("assert");
        ::cvlr_log::cvlr_log("_", &("2 > 1"));
        ::cvlr_log::cvlr_log("2", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("1", &(__cvlr_rhs));
        cvlr::log::log_scope_start("assert");
        {
            let c_ = __cvlr_lhs > __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = y;
        cvlr::log::log_scope_start("assert");
        ::cvlr_log::cvlr_log("_", &("x <= y"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("y", &(__cvlr_rhs));
        cvlr::log::log_scope_start("assert");
        {
            let c_ = __cvlr_lhs <= __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
}
