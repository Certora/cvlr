use cvlr_macros::cvlr_assert_that;
pub fn test_comparisons() {
    let a = 1;
    let b = 2;
    let x = 3;
    let y = 4;
    let p = 5;
    let q = 6;
    let m = 7;
    let n = 8;
    let c = 9;
    {
        let __cvlr_lhs = a;
        let __cvlr_rhs = b;
        cvlr::log::log_scope_start("assert");
        ::cvlr_log::cvlr_log("_", &("a < b"));
        ::cvlr_log::cvlr_log("a", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("b", &(__cvlr_rhs));
        cvlr::log::log_scope_start("assert");
        {
            let c_ = __cvlr_lhs < __cvlr_rhs;
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
    {
        let __cvlr_lhs = p;
        let __cvlr_rhs = q;
        cvlr::log::log_scope_start("assert");
        ::cvlr_log::cvlr_log("_", &("p > q"));
        ::cvlr_log::cvlr_log("p", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("q", &(__cvlr_rhs));
        cvlr::log::log_scope_start("assert");
        {
            let c_ = __cvlr_lhs > __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = m;
        let __cvlr_rhs = n;
        cvlr::log::log_scope_start("assert");
        ::cvlr_log::cvlr_log("_", &("m >= n"));
        ::cvlr_log::cvlr_log("m", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("n", &(__cvlr_rhs));
        cvlr::log::log_scope_start("assert");
        {
            let c_ = __cvlr_lhs >= __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = y;
        cvlr::log::log_scope_start("assert");
        ::cvlr_log::cvlr_log("_", &("x == y"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("y", &(__cvlr_rhs));
        cvlr::log::log_scope_start("assert");
        {
            let c_ = __cvlr_lhs == __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = a;
        let __cvlr_rhs = b;
        cvlr::log::log_scope_start("assert");
        ::cvlr_log::cvlr_log("_", &("a != b"));
        ::cvlr_log::cvlr_log("a", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("b", &(__cvlr_rhs));
        cvlr::log::log_scope_start("assert");
        {
            let c_ = __cvlr_lhs != __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = x + 1;
        let __cvlr_rhs = y * 2;
        cvlr::log::log_scope_start("assert");
        ::cvlr_log::cvlr_log("_", &("x + 1 < y * 2"));
        ::cvlr_log::cvlr_log("x + 1", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("y * 2", &(__cvlr_rhs));
        cvlr::log::log_scope_start("assert");
        {
            let c_ = __cvlr_lhs < __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = a;
        let __cvlr_rhs = c;
        cvlr::log::log_scope_start("assert");
        ::cvlr_log::cvlr_log("_", &("a > c"));
        ::cvlr_log::cvlr_log("a", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("c", &(__cvlr_rhs));
        cvlr::log::log_scope_start("assert");
        {
            let c_ = __cvlr_lhs > __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = a;
        let __cvlr_rhs = b;
        cvlr::log::log_scope_start("assert");
        ::cvlr_log::cvlr_log("_", &("a < b"));
        ::cvlr_log::cvlr_log("a", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("b", &(__cvlr_rhs));
        cvlr::log::log_scope_start("assert");
        {
            let c_ = __cvlr_lhs < __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = y;
        cvlr::log::log_scope_start("assert");
        ::cvlr_log::cvlr_log("_", &("x > y"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("y", &(__cvlr_rhs));
        cvlr::log::log_scope_start("assert");
        {
            let c_ = __cvlr_lhs > __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = p;
        let __cvlr_rhs = q;
        cvlr::log::log_scope_start("assert");
        ::cvlr_log::cvlr_log("_", &("p <= q"));
        ::cvlr_log::cvlr_log("p", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("q", &(__cvlr_rhs));
        cvlr::log::log_scope_start("assert");
        {
            let c_ = __cvlr_lhs <= __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = m;
        let __cvlr_rhs = n;
        cvlr::log::log_scope_start("assert");
        ::cvlr_log::cvlr_log("_", &("m >= n"));
        ::cvlr_log::cvlr_log("m", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("n", &(__cvlr_rhs));
        cvlr::log::log_scope_start("assert");
        {
            let c_ = __cvlr_lhs >= __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = y;
        cvlr::log::log_scope_start("assert");
        ::cvlr_log::cvlr_log("_", &("x == y"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("y", &(__cvlr_rhs));
        cvlr::log::log_scope_start("assert");
        {
            let c_ = __cvlr_lhs == __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = a;
        let __cvlr_rhs = b;
        cvlr::log::log_scope_start("assert");
        ::cvlr_log::cvlr_log("_", &("a != b"));
        ::cvlr_log::cvlr_log("a", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("b", &(__cvlr_rhs));
        cvlr::log::log_scope_start("assert");
        {
            let c_ = __cvlr_lhs != __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = a;
        let __cvlr_rhs = b;
        cvlr::log::log_scope_start("assert");
        ::cvlr_log::cvlr_log("_", &("a < b"));
        ::cvlr_log::cvlr_log("a", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("b", &(__cvlr_rhs));
        cvlr::log::log_scope_start("assert");
        {
            let c_ = __cvlr_lhs < __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = y;
        cvlr::log::log_scope_start("assert");
        ::cvlr_log::cvlr_log("_", &("x > y"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("y", &(__cvlr_rhs));
        cvlr::log::log_scope_start("assert");
        {
            let c_ = __cvlr_lhs > __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
}
pub fn main() {}
