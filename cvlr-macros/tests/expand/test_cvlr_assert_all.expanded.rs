use cvlr_macros::cvlr_assert_all;
pub fn test_assert_all_comma_separated() {
    let x = 5;
    let y = 10;
    let z = 15;
    let a = 1;
    let b = 2;
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assert x > 0"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
        {
            let c_ = __cvlr_lhs > __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = y;
        let __cvlr_rhs = 20;
        ::cvlr_log::cvlr_log("_", &("assert y < 20"));
        ::cvlr_log::cvlr_log("y", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("20", &(__cvlr_rhs));
        {
            let c_ = __cvlr_lhs < __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = z;
        let __cvlr_rhs = x;
        ::cvlr_log::cvlr_log("_", &("assert z > x"));
        ::cvlr_log::cvlr_log("z", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("x", &(__cvlr_rhs));
        {
            let c_ = __cvlr_lhs > __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = a;
        let __cvlr_rhs = b;
        ::cvlr_log::cvlr_log("_", &("assert a < b"));
        ::cvlr_log::cvlr_log("a", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("b", &(__cvlr_rhs));
        {
            let c_ = __cvlr_lhs < __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = 5;
        ::cvlr_log::cvlr_log("_", &("assert x == 5"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("5", &(__cvlr_rhs));
        {
            let c_ = __cvlr_lhs == __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = y;
        let __cvlr_rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assert y != 0"));
        ::cvlr_log::cvlr_log("y", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
        {
            let c_ = __cvlr_lhs != __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assert x > 0"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
        {
            let c_ = __cvlr_lhs > __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = y;
        let __cvlr_rhs = 20;
        ::cvlr_log::cvlr_log("_", &("assert y < 20"));
        ::cvlr_log::cvlr_log("y", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("20", &(__cvlr_rhs));
        {
            let c_ = __cvlr_lhs < __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = z;
        let __cvlr_rhs = x;
        ::cvlr_log::cvlr_log("_", &("assert z > x"));
        ::cvlr_log::cvlr_log("z", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("x", &(__cvlr_rhs));
        {
            let c_ = __cvlr_lhs > __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = a;
        let __cvlr_rhs = b;
        ::cvlr_log::cvlr_log("_", &("assert a < b"));
        ::cvlr_log::cvlr_log("a", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("b", &(__cvlr_rhs));
        {
            let c_ = __cvlr_lhs < __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = 5;
        ::cvlr_log::cvlr_log("_", &("assert x == 5"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("5", &(__cvlr_rhs));
        {
            let c_ = __cvlr_lhs == __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = y;
        let __cvlr_rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assert y != 0"));
        ::cvlr_log::cvlr_log("y", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
        {
            let c_ = __cvlr_lhs != __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
}
pub fn test_assert_all_semicolon_separated() {
    let x = 5;
    let y = 10;
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assert x > 0"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
        {
            let c_ = __cvlr_lhs > __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = y;
        let __cvlr_rhs = 20;
        ::cvlr_log::cvlr_log("_", &("assert y < 20"));
        ::cvlr_log::cvlr_log("y", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("20", &(__cvlr_rhs));
        {
            let c_ = __cvlr_lhs < __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = y;
        ::cvlr_log::cvlr_log("_", &("assert x < y"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("y", &(__cvlr_rhs));
        {
            let c_ = __cvlr_lhs < __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
}
pub fn test_assert_all_mixed_separators() {
    let x = 5;
    let y = 10;
    let flag = true;
    let c = 3;
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assert x > 0"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
        {
            let c_ = __cvlr_lhs > __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = y;
        let __cvlr_rhs = 20;
        ::cvlr_log::cvlr_log("_", &("assert y < 20"));
        ::cvlr_log::cvlr_log("y", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("20", &(__cvlr_rhs));
        {
            let c_ = __cvlr_lhs < __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_guard = flag;
        ::cvlr_log::cvlr_log("_", &("assert if flag { x < y }"));
        ::cvlr_log::cvlr_log("flag", &(__cvlr_guard));
        if __cvlr_guard {
            let __cvlr_lhs = x;
            let __cvlr_rhs = y;
            ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("y", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs < __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assert x > 0"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
        {
            let c_ = __cvlr_lhs > __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = y;
        let __cvlr_rhs = 20;
        ::cvlr_log::cvlr_log("_", &("assert y < 20"));
        ::cvlr_log::cvlr_log("y", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("20", &(__cvlr_rhs));
        {
            let c_ = __cvlr_lhs < __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_guard = flag;
        ::cvlr_log::cvlr_log("_", &("assert if flag { c < y }"));
        ::cvlr_log::cvlr_log("flag", &(__cvlr_guard));
        if __cvlr_guard {
            let __cvlr_lhs = c;
            let __cvlr_rhs = y;
            ::cvlr_log::cvlr_log("c", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("y", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs < __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
}
pub fn test_assert_all_guarded() {
    let flag = true;
    let a = 1;
    let b = 2;
    let x = 5;
    let y = 10;
    {
        let __cvlr_guard = flag;
        ::cvlr_log::cvlr_log("_", &("assert if flag { a < b }"));
        ::cvlr_log::cvlr_log("flag", &(__cvlr_guard));
        if __cvlr_guard {
            let __cvlr_lhs = a;
            let __cvlr_rhs = b;
            ::cvlr_log::cvlr_log("a", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("b", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs < __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    {
        let __cvlr_guard = x > 0;
        ::cvlr_log::cvlr_log("_", &("assert if x > 0 { y < 20 }"));
        ::cvlr_log::cvlr_log("x > 0", &(__cvlr_guard));
        if __cvlr_guard {
            let __cvlr_lhs = y;
            let __cvlr_rhs = 20;
            ::cvlr_log::cvlr_log("y", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("20", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs < __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    {
        let __cvlr_guard = flag;
        ::cvlr_log::cvlr_log("_", &("assert if flag { a < b }"));
        ::cvlr_log::cvlr_log("flag", &(__cvlr_guard));
        if __cvlr_guard {
            let __cvlr_lhs = a;
            let __cvlr_rhs = b;
            ::cvlr_log::cvlr_log("a", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("b", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs < __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    {
        let __cvlr_guard = x > 0;
        ::cvlr_log::cvlr_log("_", &("assert if x > 0 { y < 20 }"));
        ::cvlr_log::cvlr_log("x > 0", &(__cvlr_guard));
        if __cvlr_guard {
            let __cvlr_lhs = y;
            let __cvlr_rhs = 20;
            ::cvlr_log::cvlr_log("y", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("20", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs < __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
}
pub fn test_assert_all_mixed_guarded_unguarded() {
    let x = 5;
    let y = 10;
    let flag = true;
    let a = 1;
    let b = 2;
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assert x > 0"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
        {
            let c_ = __cvlr_lhs > __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_guard = flag;
        ::cvlr_log::cvlr_log("_", &("assert if flag { x < y }"));
        ::cvlr_log::cvlr_log("flag", &(__cvlr_guard));
        if __cvlr_guard {
            let __cvlr_lhs = x;
            let __cvlr_rhs = y;
            ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("y", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs < __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    {
        let __cvlr_guard = flag;
        ::cvlr_log::cvlr_log("_", &("assert if flag { a < b }"));
        ::cvlr_log::cvlr_log("flag", &(__cvlr_guard));
        if __cvlr_guard {
            let __cvlr_lhs = a;
            let __cvlr_rhs = b;
            ::cvlr_log::cvlr_log("a", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("b", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs < __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    {
        let __cvlr_lhs = y;
        let __cvlr_rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assert y > 0"));
        ::cvlr_log::cvlr_log("y", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
        {
            let c_ = __cvlr_lhs > __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assert x > 0"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
        {
            let c_ = __cvlr_lhs > __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let __cvlr_guard = flag;
        ::cvlr_log::cvlr_log("_", &("assert if flag { a < b }"));
        ::cvlr_log::cvlr_log("flag", &(__cvlr_guard));
        if __cvlr_guard {
            let __cvlr_lhs = a;
            let __cvlr_rhs = b;
            ::cvlr_log::cvlr_log("a", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("b", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs < __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    {
        let __cvlr_lhs = y;
        let __cvlr_rhs = 20;
        ::cvlr_log::cvlr_log("_", &("assert y < 20"));
        ::cvlr_log::cvlr_log("y", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("20", &(__cvlr_rhs));
        {
            let c_ = __cvlr_lhs < __cvlr_rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
}
pub fn test_assert_all_boolean_expressions() {
    let flag = true;
    let x = 5;
    let y = 3;
    let z = 7;
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
        let __cvlr_guard = flag;
        ::cvlr_log::cvlr_log("_", &("assert if flag { x > 0 }"));
        ::cvlr_log::cvlr_log("flag", &(__cvlr_guard));
        if __cvlr_guard {
            let __cvlr_lhs = x;
            let __cvlr_rhs = 0;
            ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs > __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    if x > 0 {
        {
            let c_ = y > 0 && z < 10;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    }
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
        let c_ = flag;
        ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
        ::cvlr_asserts::cvlr_assert_checked(c_);
    };
    {
        let c_ = x > 0 && y < 10;
        ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
        ::cvlr_asserts::cvlr_assert_checked(c_);
    };
}
pub fn main() {}
