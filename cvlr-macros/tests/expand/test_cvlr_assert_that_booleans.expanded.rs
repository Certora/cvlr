use cvlr_macros::cvlr_assert_that;
pub fn test() {
    let flag = true;
    let x = 5;
    let y = 3;
    let a = true;
    let b = false;
    let condition = false;
    let guard = true;
    let z = 7;
    let error = false;
    let test = true;
    let c = true;
    ();
    {
        let c_ = flag;
        ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
        ::cvlr_asserts::cvlr_assert_checked(c_);
    };
    {
        let c_ = x > 0 && y < 10;
        ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
        ::cvlr_asserts::cvlr_assert_checked(c_);
    };
    {
        let c_ = a || b;
        ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
        ::cvlr_asserts::cvlr_assert_checked(c_);
    };
    {
        let c_ = !condition;
        ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
        ::cvlr_asserts::cvlr_assert_checked(c_);
    };
    {
        let __cvlr_lhs = x + y;
        let __cvlr_rhs = 0;
        cvlr::log::log_scope_start("assert");
        ::cvlr_log::cvlr_log("_", &("x + y > 0"));
        ::cvlr_log::cvlr_log("x + y", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
        cvlr::log::log_scope_start("assert");
        {
            let c_ = __cvlr_lhs > __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let c_ = flag;
        ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
        ::cvlr_asserts::cvlr_assert_checked(c_);
    };
    {
        let c_ = x > 0 && y < 10;
        ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
        ::cvlr_asserts::cvlr_assert_checked(c_);
    };
    {
        let c_ = a || b;
        ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
        ::cvlr_asserts::cvlr_assert_checked(c_);
    };
    if guard {
        {
            let c_ = condition;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    } else {
        ()
    };
    if x > 0 {
        {
            let c_ = y > 0 && z < 10;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    } else {
        ()
    };
    if flag {
        {
            let c_ = !error;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    } else {
        ()
    };
    if test {
        {
            let c_ = (a || b) && c;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    } else {
        ()
    };
}
fn main() {}
