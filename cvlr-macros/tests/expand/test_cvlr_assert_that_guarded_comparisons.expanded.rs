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
        let guard = flag;
        ::cvlr_log::cvlr_log("_", &("assert flag == > a < b"));
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
        ::cvlr_log::cvlr_log("_", &("assert x > 0 == > y <= z"));
        ::cvlr_log::cvlr_log("x > 0", &(guard));
        if guard {
            let lhs = y;
            let rhs = z;
            ::cvlr_log::cvlr_log("y", &(lhs));
            ::cvlr_log::cvlr_log("z", &(rhs));
            {
                let c_ = lhs <= rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    {
        let guard = cond;
        ::cvlr_log::cvlr_log("_", &("assert cond == > p > q"));
        ::cvlr_log::cvlr_log("cond", &(guard));
        if guard {
            let lhs = p;
            let rhs = q;
            ::cvlr_log::cvlr_log("p", &(lhs));
            ::cvlr_log::cvlr_log("q", &(rhs));
            {
                let c_ = lhs > rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    {
        let guard = guard;
        ::cvlr_log::cvlr_log("_", &("assert guard == > m >= n"));
        ::cvlr_log::cvlr_log("guard", &(guard));
        if guard {
            let lhs = m;
            let rhs = n;
            ::cvlr_log::cvlr_log("m", &(lhs));
            ::cvlr_log::cvlr_log("n", &(rhs));
            {
                let c_ = lhs >= rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    {
        let guard = test;
        ::cvlr_log::cvlr_log("_", &("assert test == > x == y"));
        ::cvlr_log::cvlr_log("test", &(guard));
        if guard {
            let lhs = x;
            let rhs = y;
            ::cvlr_log::cvlr_log("x", &(lhs));
            ::cvlr_log::cvlr_log("y", &(rhs));
            {
                let c_ = lhs == rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    {
        let guard = check;
        ::cvlr_log::cvlr_log("_", &("assert check == > a != b"));
        ::cvlr_log::cvlr_log("check", &(guard));
        if guard {
            let lhs = a;
            let rhs = b;
            ::cvlr_log::cvlr_log("a", &(lhs));
            ::cvlr_log::cvlr_log("b", &(rhs));
            {
                let c_ = lhs != rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    {
        let guard = a > c;
        ::cvlr_log::cvlr_log("_", &("assert a > c == > d < p"));
        ::cvlr_log::cvlr_log("a > c", &(guard));
        if guard {
            let lhs = d;
            let rhs = p;
            ::cvlr_log::cvlr_log("d", &(lhs));
            ::cvlr_log::cvlr_log("p", &(rhs));
            {
                let c_ = lhs < rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
    {
        let guard = x + 1 > 0;
        ::cvlr_log::cvlr_log("_", &("assert x + 1 > 0 == > y * 2 < z"));
        ::cvlr_log::cvlr_log("x + 1 > 0", &(guard));
        if guard {
            let lhs = y * 2;
            let rhs = z;
            ::cvlr_log::cvlr_log("y * 2", &(lhs));
            ::cvlr_log::cvlr_log("z", &(rhs));
            {
                let c_ = lhs < rhs;
                ::cvlr_asserts::log::add_loc("<FILE>", 0u32);
                ::cvlr_asserts::cvlr_assert_checked(c_);
            };
        }
    };
}
pub fn main() {}
