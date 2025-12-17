use cvlr_macros::cvlr_assume_that;
pub fn test_assume_comparisons() {
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
        ::cvlr_log::cvlr_log("_", &("assume a < b"));
        ::cvlr_log::cvlr_log("a", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("b", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
    };
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = y;
        ::cvlr_log::cvlr_log("_", &("assume x <= y"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("y", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs <= __cvlr_rhs);
    };
    {
        let __cvlr_lhs = p;
        let __cvlr_rhs = q;
        ::cvlr_log::cvlr_log("_", &("assume p > q"));
        ::cvlr_log::cvlr_log("p", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("q", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs > __cvlr_rhs);
    };
    {
        let __cvlr_lhs = m;
        let __cvlr_rhs = n;
        ::cvlr_log::cvlr_log("_", &("assume m >= n"));
        ::cvlr_log::cvlr_log("m", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("n", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs >= __cvlr_rhs);
    };
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = y;
        ::cvlr_log::cvlr_log("_", &("assume x == y"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("y", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs == __cvlr_rhs);
    };
    {
        let __cvlr_lhs = a;
        let __cvlr_rhs = b;
        ::cvlr_log::cvlr_log("_", &("assume a != b"));
        ::cvlr_log::cvlr_log("a", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("b", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs != __cvlr_rhs);
    };
    {
        let __cvlr_lhs = x + 1;
        let __cvlr_rhs = y * 2;
        ::cvlr_log::cvlr_log("_", &("assume x + 1 < y * 2"));
        ::cvlr_log::cvlr_log("x + 1", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("y * 2", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
    };
    {
        let __cvlr_lhs = a;
        let __cvlr_rhs = c;
        ::cvlr_log::cvlr_log("_", &("assume a > c"));
        ::cvlr_log::cvlr_log("a", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("c", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs > __cvlr_rhs);
    };
    {
        let __cvlr_lhs = a;
        let __cvlr_rhs = b;
        ::cvlr_log::cvlr_log("_", &("assume a < b"));
        ::cvlr_log::cvlr_log("a", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("b", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
    };
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = y;
        ::cvlr_log::cvlr_log("_", &("assume x > y"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("y", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs > __cvlr_rhs);
    };
    {
        let __cvlr_lhs = p;
        let __cvlr_rhs = q;
        ::cvlr_log::cvlr_log("_", &("assume p <= q"));
        ::cvlr_log::cvlr_log("p", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("q", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs <= __cvlr_rhs);
    };
    {
        let __cvlr_lhs = m;
        let __cvlr_rhs = n;
        ::cvlr_log::cvlr_log("_", &("assume m >= n"));
        ::cvlr_log::cvlr_log("m", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("n", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs >= __cvlr_rhs);
    };
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = y;
        ::cvlr_log::cvlr_log("_", &("assume x == y"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("y", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs == __cvlr_rhs);
    };
    {
        let __cvlr_lhs = a;
        let __cvlr_rhs = b;
        ::cvlr_log::cvlr_log("_", &("assume a != b"));
        ::cvlr_log::cvlr_log("a", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("b", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs != __cvlr_rhs);
    };
    {
        let __cvlr_lhs = a;
        let __cvlr_rhs = b;
        ::cvlr_log::cvlr_log("_", &("assume a < b"));
        ::cvlr_log::cvlr_log("a", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("b", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
    };
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = y;
        ::cvlr_log::cvlr_log("_", &("assume x > y"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("y", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs > __cvlr_rhs);
    };
}
pub fn test_assume_guarded_comparisons() {
    let flag = true;
    let a = 1;
    let b = 2;
    let x = 5;
    let y = 10;
    if flag {
        {
            let __cvlr_lhs = a;
            let __cvlr_rhs = b;
            ::cvlr_log::cvlr_log("_", &("assume a < b"));
            ::cvlr_log::cvlr_log("a", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("b", &(__cvlr_rhs));
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
        };
    }
    if x > 0 {
        {
            let __cvlr_lhs = y;
            let __cvlr_rhs = 20;
            ::cvlr_log::cvlr_log("_", &("assume y <= 20"));
            ::cvlr_log::cvlr_log("y", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("20", &(__cvlr_rhs));
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs <= __cvlr_rhs);
        };
    }
    if flag {
        {
            let __cvlr_lhs = a;
            let __cvlr_rhs = b;
            ::cvlr_log::cvlr_log("_", &("assume a < b"));
            ::cvlr_log::cvlr_log("a", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("b", &(__cvlr_rhs));
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
        };
    }
    if x > 0 {
        {
            let __cvlr_lhs = y;
            let __cvlr_rhs = 20;
            ::cvlr_log::cvlr_log("_", &("assume y <= 20"));
            ::cvlr_log::cvlr_log("y", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("20", &(__cvlr_rhs));
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs <= __cvlr_rhs);
        };
    }
    if flag {
        {
            let __cvlr_lhs = a;
            let __cvlr_rhs = b;
            ::cvlr_log::cvlr_log("_", &("assume a < b"));
            ::cvlr_log::cvlr_log("a", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("b", &(__cvlr_rhs));
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
        };
    }
}
pub fn test_assume_booleans() {
    let flag = true;
    let x = 5;
    let y = 3;
    ::cvlr_asserts::cvlr_assume_checked(flag);
    ::cvlr_asserts::cvlr_assume_checked(x > 0 && y < 10);
    if flag {
        {
            let __cvlr_lhs = x;
            let __cvlr_rhs = 0;
            ::cvlr_log::cvlr_log("_", &("assume x > 0"));
            ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs > __cvlr_rhs);
        };
    }
    if x > 0 {
        ::cvlr_asserts::cvlr_assume_checked(y > 0 && y < 10);
    }
}
pub fn main() {}
