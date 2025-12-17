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
        ::cvlr_log::cvlr_log("_", &("assert if x > 0 { y <= z }"));
        ::cvlr_log::cvlr_log("x > 0", &(__cvlr_guard));
        if __cvlr_guard {
            let __cvlr_lhs = y;
            let __cvlr_rhs = z;
            ::cvlr_log::cvlr_log("y", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("z", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs <= __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    {
        let __cvlr_guard = cond;
        ::cvlr_log::cvlr_log("_", &("assert if cond { p > q }"));
        ::cvlr_log::cvlr_log("cond", &(__cvlr_guard));
        if __cvlr_guard {
            let __cvlr_lhs = p;
            let __cvlr_rhs = q;
            ::cvlr_log::cvlr_log("p", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("q", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs > __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    {
        let __cvlr_guard = guard;
        ::cvlr_log::cvlr_log("_", &("assert if guard { m >= n }"));
        ::cvlr_log::cvlr_log("guard", &(__cvlr_guard));
        if __cvlr_guard {
            let __cvlr_lhs = m;
            let __cvlr_rhs = n;
            ::cvlr_log::cvlr_log("m", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("n", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs >= __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    {
        let __cvlr_guard = test;
        ::cvlr_log::cvlr_log("_", &("assert if test { x == y }"));
        ::cvlr_log::cvlr_log("test", &(__cvlr_guard));
        if __cvlr_guard {
            let __cvlr_lhs = x;
            let __cvlr_rhs = y;
            ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("y", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs == __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    {
        let __cvlr_guard = check;
        ::cvlr_log::cvlr_log("_", &("assert if check { a != b }"));
        ::cvlr_log::cvlr_log("check", &(__cvlr_guard));
        if __cvlr_guard {
            let __cvlr_lhs = a;
            let __cvlr_rhs = b;
            ::cvlr_log::cvlr_log("a", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("b", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs != __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    {
        let __cvlr_guard = a > c;
        ::cvlr_log::cvlr_log("_", &("assert if a > c { d < p }"));
        ::cvlr_log::cvlr_log("a > c", &(__cvlr_guard));
        if __cvlr_guard {
            let __cvlr_lhs = d;
            let __cvlr_rhs = p;
            ::cvlr_log::cvlr_log("d", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("p", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs < __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    {
        let __cvlr_guard = x + 1 > 0;
        ::cvlr_log::cvlr_log("_", &("assert if x + 1 > 0 { y * 2 < z }"));
        ::cvlr_log::cvlr_log("x + 1 > 0", &(__cvlr_guard));
        if __cvlr_guard {
            let __cvlr_lhs = y * 2;
            let __cvlr_rhs = z;
            ::cvlr_log::cvlr_log("y * 2", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("z", &(__cvlr_rhs));
            {
                let c_ = __cvlr_lhs < __cvlr_rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
}
pub fn main() {}
