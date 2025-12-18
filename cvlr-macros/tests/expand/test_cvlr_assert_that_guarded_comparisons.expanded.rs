use cvlr_macros::cvlr_assert_that;
pub fn test_guarded_comparisons() {
    let flag = true;
    let cond = false;
    let guard = true;
    let test = true;
    let check = false;
    let a = 1;
    let b = 2;
    let c = 3;
    let d = 4;
    let p = 5;
    let q = 6;
    let x = 6;
    let y = 7;
    let z = 8;
    let m = 9;
    let n = 10;
    {
        let c_ = if flag { a < b } else { true };
        ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
        ::cvlr_asserts::cvlr_assert_checked(c_);
    };
    {
        let c_ = if x > 0 { y <= z } else { true };
        ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
        ::cvlr_asserts::cvlr_assert_checked(c_);
    };
    {
        let c_ = if cond { p > q } else { true };
        ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
        ::cvlr_asserts::cvlr_assert_checked(c_);
    };
    {
        let c_ = if guard { m >= n } else { true };
        ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
        ::cvlr_asserts::cvlr_assert_checked(c_);
    };
    {
        let c_ = if test { x == y } else { true };
        ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
        ::cvlr_asserts::cvlr_assert_checked(c_);
    };
    {
        let c_ = if check { a != b } else { true };
        ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
        ::cvlr_asserts::cvlr_assert_checked(c_);
    };
    {
        let c_ = if a > c { d < p } else { true };
        ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
        ::cvlr_asserts::cvlr_assert_checked(c_);
    };
    {
        let c_ = if x + 1 > 0 { y * 2 < z } else { true };
        ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
        ::cvlr_asserts::cvlr_assert_checked(c_);
    };
}
pub fn main() {}
