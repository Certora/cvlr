use cvlr_macros::cvlr_assert_that;
pub fn test_true_literal() {
    ();
}
pub fn test_if_else_expressions() {
    let guard = true;
    let flag = false;
    let x = 5;
    let y = 10;
    let a = 1;
    let b = 2;
    if guard {
        {
            let c_ = flag;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    } else {
        ()
    };
    if guard {
        {
            let c_ = x > 0;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    } else {
        {
            let c_ = y > 0;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    if guard {
        {
            let c_ = a < b;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    } else {
        {
            let c_ = b > a;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    if guard {
        if flag {
            {
                let c_ = x > 0;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        } else {
            {
                let c_ = y > 0;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    } else {
        ()
    };
    if guard {
        ()
    } else {
        {
            let c_ = flag;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    if guard {
        {
            let c_ = flag;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    } else {
        ()
    };
    if guard { () } else { () };
}
pub fn test_if_without_else() {
    let guard = true;
    let flag = false;
    let x = 5;
    if guard {
        {
            let c_ = flag;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    }
    if guard {
        {
            let c_ = x > 0;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    }
    if guard {
        ()
    }
}
pub fn main() {}
