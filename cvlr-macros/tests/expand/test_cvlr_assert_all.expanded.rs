use cvlr_macros::cvlr_assert_all;
pub fn test_assert_all_comma_separated() {
    let x = 5;
    let y = 10;
    let z = 15;
    let a = 1;
    let b = 2;
    {
        let lhs = x;
        let rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assert x > 0"));
        ::cvlr_log::cvlr_log("x", &(lhs));
        ::cvlr_log::cvlr_log("0", &(rhs));
        {
            let c_ = lhs > rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let lhs = y;
        let rhs = 20;
        ::cvlr_log::cvlr_log("_", &("assert y < 20"));
        ::cvlr_log::cvlr_log("y", &(lhs));
        ::cvlr_log::cvlr_log("20", &(rhs));
        {
            let c_ = lhs < rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let lhs = z;
        let rhs = x;
        ::cvlr_log::cvlr_log("_", &("assert z > x"));
        ::cvlr_log::cvlr_log("z", &(lhs));
        ::cvlr_log::cvlr_log("x", &(rhs));
        {
            let c_ = lhs > rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
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
        let rhs = 5;
        ::cvlr_log::cvlr_log("_", &("assert x == 5"));
        ::cvlr_log::cvlr_log("x", &(lhs));
        ::cvlr_log::cvlr_log("5", &(rhs));
        {
            let c_ = lhs == rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let lhs = y;
        let rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assert y != 0"));
        ::cvlr_log::cvlr_log("y", &(lhs));
        ::cvlr_log::cvlr_log("0", &(rhs));
        {
            let c_ = lhs != rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
}
pub fn test_assert_all_semicolon_separated() {
    let x = 5;
    let y = 10;
    {
        let lhs = x;
        let rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assert x > 0"));
        ::cvlr_log::cvlr_log("x", &(lhs));
        ::cvlr_log::cvlr_log("0", &(rhs));
        {
            let c_ = lhs > rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let lhs = y;
        let rhs = 20;
        ::cvlr_log::cvlr_log("_", &("assert y < 20"));
        ::cvlr_log::cvlr_log("y", &(lhs));
        ::cvlr_log::cvlr_log("20", &(rhs));
        {
            let c_ = lhs < rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let lhs = x;
        let rhs = y;
        ::cvlr_log::cvlr_log("_", &("assert x < y"));
        ::cvlr_log::cvlr_log("x", &(lhs));
        ::cvlr_log::cvlr_log("y", &(rhs));
        {
            let c_ = lhs < rhs;
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
        let lhs = x;
        let rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assert x > 0"));
        ::cvlr_log::cvlr_log("x", &(lhs));
        ::cvlr_log::cvlr_log("0", &(rhs));
        {
            let c_ = lhs > rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let lhs = y;
        let rhs = 20;
        ::cvlr_log::cvlr_log("_", &("assert y < 20"));
        ::cvlr_log::cvlr_log("y", &(lhs));
        ::cvlr_log::cvlr_log("20", &(rhs));
        {
            let c_ = lhs < rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let guard = flag;
        ::cvlr_log::cvlr_log("_", &("assert if flag { x < y }"));
        ::cvlr_log::cvlr_log("flag", &(guard));
        if guard {
            let lhs = x;
            let rhs = y;
            ::cvlr_log::cvlr_log("x", &(lhs));
            ::cvlr_log::cvlr_log("y", &(rhs));
            {
                let c_ = lhs < rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    {
        let lhs = x;
        let rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assert x > 0"));
        ::cvlr_log::cvlr_log("x", &(lhs));
        ::cvlr_log::cvlr_log("0", &(rhs));
        {
            let c_ = lhs > rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let lhs = y;
        let rhs = 20;
        ::cvlr_log::cvlr_log("_", &("assert y < 20"));
        ::cvlr_log::cvlr_log("y", &(lhs));
        ::cvlr_log::cvlr_log("20", &(rhs));
        {
            let c_ = lhs < rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let guard = flag;
        ::cvlr_log::cvlr_log("_", &("assert if flag { c < y }"));
        ::cvlr_log::cvlr_log("flag", &(guard));
        if guard {
            let lhs = c;
            let rhs = y;
            ::cvlr_log::cvlr_log("c", &(lhs));
            ::cvlr_log::cvlr_log("y", &(rhs));
            {
                let c_ = lhs < rhs;
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
        let guard = flag;
        ::cvlr_log::cvlr_log("_", &("assert if flag { a < b }"));
        ::cvlr_log::cvlr_log("flag", &(guard));
        if guard {
            let lhs = a;
            let rhs = b;
            ::cvlr_log::cvlr_log("a", &(lhs));
            ::cvlr_log::cvlr_log("b", &(rhs));
            {
                let c_ = lhs < rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    {
        let guard = x > 0;
        ::cvlr_log::cvlr_log("_", &("assert if x > 0 { y < 20 }"));
        ::cvlr_log::cvlr_log("x > 0", &(guard));
        if guard {
            let lhs = y;
            let rhs = 20;
            ::cvlr_log::cvlr_log("y", &(lhs));
            ::cvlr_log::cvlr_log("20", &(rhs));
            {
                let c_ = lhs < rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    {
        let guard = flag;
        ::cvlr_log::cvlr_log("_", &("assert if flag { a < b }"));
        ::cvlr_log::cvlr_log("flag", &(guard));
        if guard {
            let lhs = a;
            let rhs = b;
            ::cvlr_log::cvlr_log("a", &(lhs));
            ::cvlr_log::cvlr_log("b", &(rhs));
            {
                let c_ = lhs < rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    {
        let guard = x > 0;
        ::cvlr_log::cvlr_log("_", &("assert if x > 0 { y < 20 }"));
        ::cvlr_log::cvlr_log("x > 0", &(guard));
        if guard {
            let lhs = y;
            let rhs = 20;
            ::cvlr_log::cvlr_log("y", &(lhs));
            ::cvlr_log::cvlr_log("20", &(rhs));
            {
                let c_ = lhs < rhs;
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
        let lhs = x;
        let rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assert x > 0"));
        ::cvlr_log::cvlr_log("x", &(lhs));
        ::cvlr_log::cvlr_log("0", &(rhs));
        {
            let c_ = lhs > rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let guard = flag;
        ::cvlr_log::cvlr_log("_", &("assert if flag { x < y }"));
        ::cvlr_log::cvlr_log("flag", &(guard));
        if guard {
            let lhs = x;
            let rhs = y;
            ::cvlr_log::cvlr_log("x", &(lhs));
            ::cvlr_log::cvlr_log("y", &(rhs));
            {
                let c_ = lhs < rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    {
        let guard = flag;
        ::cvlr_log::cvlr_log("_", &("assert if flag { a < b }"));
        ::cvlr_log::cvlr_log("flag", &(guard));
        if guard {
            let lhs = a;
            let rhs = b;
            ::cvlr_log::cvlr_log("a", &(lhs));
            ::cvlr_log::cvlr_log("b", &(rhs));
            {
                let c_ = lhs < rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    {
        let lhs = y;
        let rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assert y > 0"));
        ::cvlr_log::cvlr_log("y", &(lhs));
        ::cvlr_log::cvlr_log("0", &(rhs));
        {
            let c_ = lhs > rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let lhs = x;
        let rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assert x > 0"));
        ::cvlr_log::cvlr_log("x", &(lhs));
        ::cvlr_log::cvlr_log("0", &(rhs));
        {
            let c_ = lhs > rhs;
            ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
            ::cvlr_asserts::cvlr_assert_checked(c_);
        };
    };
    {
        let guard = flag;
        ::cvlr_log::cvlr_log("_", &("assert if flag { a < b }"));
        ::cvlr_log::cvlr_log("flag", &(guard));
        if guard {
            let lhs = a;
            let rhs = b;
            ::cvlr_log::cvlr_log("a", &(lhs));
            ::cvlr_log::cvlr_log("b", &(rhs));
            {
                let c_ = lhs < rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    {
        let lhs = y;
        let rhs = 20;
        ::cvlr_log::cvlr_log("_", &("assert y < 20"));
        ::cvlr_log::cvlr_log("y", &(lhs));
        ::cvlr_log::cvlr_log("20", &(rhs));
        {
            let c_ = lhs < rhs;
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
        let guard = flag;
        ::cvlr_log::cvlr_log("_", &("assert if flag { x > 0 }"));
        ::cvlr_log::cvlr_log("flag", &(guard));
        if guard {
            let lhs = x;
            let rhs = 0;
            ::cvlr_log::cvlr_log("x", &(lhs));
            ::cvlr_log::cvlr_log("0", &(rhs));
            {
                let c_ = lhs > rhs;
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
}
pub fn main() {}
