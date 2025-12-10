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
        let lhs = a;
        let rhs = b;
        ::cvlr_log::cvlr_log("_", &("assert a < b"));
        ::cvlr_log::cvlr_log("a", &(lhs));
        ::cvlr_log::cvlr_log("b", &(rhs));
        {
            let c_ = lhs < rhs;
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
    {
        let lhs = p;
        let rhs = q;
        ::cvlr_log::cvlr_log("_", &("assert p > q"));
        ::cvlr_log::cvlr_log("p", &(lhs));
        ::cvlr_log::cvlr_log("q", &(rhs));
        {
            let c_ = lhs > rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let lhs = m;
        let rhs = n;
        ::cvlr_log::cvlr_log("_", &("assert m >= n"));
        ::cvlr_log::cvlr_log("m", &(lhs));
        ::cvlr_log::cvlr_log("n", &(rhs));
        {
            let c_ = lhs >= rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let lhs = x;
        let rhs = y;
        ::cvlr_log::cvlr_log("_", &("assert x == y"));
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
        ::cvlr_log::cvlr_log("_", &("assert a != b"));
        ::cvlr_log::cvlr_log("a", &(lhs));
        ::cvlr_log::cvlr_log("b", &(rhs));
        {
            let c_ = lhs != rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let lhs = x + 1;
        let rhs = y * 2;
        ::cvlr_log::cvlr_log("_", &("assert x + 1 < y * 2"));
        ::cvlr_log::cvlr_log("x + 1", &(lhs));
        ::cvlr_log::cvlr_log("y * 2", &(rhs));
        {
            let c_ = lhs < rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let lhs = a;
        let rhs = c;
        ::cvlr_log::cvlr_log("_", &("assert a > c"));
        ::cvlr_log::cvlr_log("a", &(lhs));
        ::cvlr_log::cvlr_log("c", &(rhs));
        {
            let c_ = lhs > rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
}
pub fn main() {}
