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
        let lhs = a;
        let rhs = b;
        ::cvlr_log::cvlr_log("_", &("assume a < b"));
        ::cvlr_log::cvlr_log("a", &(lhs));
        ::cvlr_log::cvlr_log("b", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs < rhs);
    };
    {
        let lhs = x;
        let rhs = y;
        ::cvlr_log::cvlr_log("_", &("assume x <= y"));
        ::cvlr_log::cvlr_log("x", &(lhs));
        ::cvlr_log::cvlr_log("y", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs <= rhs);
    };
    {
        let lhs = p;
        let rhs = q;
        ::cvlr_log::cvlr_log("_", &("assume p > q"));
        ::cvlr_log::cvlr_log("p", &(lhs));
        ::cvlr_log::cvlr_log("q", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs > rhs);
    };
    {
        let lhs = m;
        let rhs = n;
        ::cvlr_log::cvlr_log("_", &("assume m >= n"));
        ::cvlr_log::cvlr_log("m", &(lhs));
        ::cvlr_log::cvlr_log("n", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs >= rhs);
    };
    {
        let lhs = x;
        let rhs = y;
        ::cvlr_log::cvlr_log("_", &("assume x == y"));
        ::cvlr_log::cvlr_log("x", &(lhs));
        ::cvlr_log::cvlr_log("y", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs == rhs);
    };
    {
        let lhs = a;
        let rhs = b;
        ::cvlr_log::cvlr_log("_", &("assume a != b"));
        ::cvlr_log::cvlr_log("a", &(lhs));
        ::cvlr_log::cvlr_log("b", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs != rhs);
    };
    {
        let lhs = x + 1;
        let rhs = y * 2;
        ::cvlr_log::cvlr_log("_", &("assume x + 1 < y * 2"));
        ::cvlr_log::cvlr_log("x + 1", &(lhs));
        ::cvlr_log::cvlr_log("y * 2", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs < rhs);
    };
    {
        let lhs = a;
        let rhs = c;
        ::cvlr_log::cvlr_log("_", &("assume a > c"));
        ::cvlr_log::cvlr_log("a", &(lhs));
        ::cvlr_log::cvlr_log("c", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs > rhs);
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
            let lhs = a;
            let rhs = b;
            ::cvlr_log::cvlr_log("_", &("assume a < b"));
            ::cvlr_log::cvlr_log("a", &(lhs));
            ::cvlr_log::cvlr_log("b", &(rhs));
            ::cvlr_asserts::cvlr_assume_checked(lhs < rhs);
        };
    }
    if x > 0 {
        {
            let lhs = y;
            let rhs = 20;
            ::cvlr_log::cvlr_log("_", &("assume y <= 20"));
            ::cvlr_log::cvlr_log("y", &(lhs));
            ::cvlr_log::cvlr_log("20", &(rhs));
            ::cvlr_asserts::cvlr_assume_checked(lhs <= rhs);
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
            let lhs = x;
            let rhs = 0;
            ::cvlr_log::cvlr_log("_", &("assume x > 0"));
            ::cvlr_log::cvlr_log("x", &(lhs));
            ::cvlr_log::cvlr_log("0", &(rhs));
            ::cvlr_asserts::cvlr_assume_checked(lhs > rhs);
        };
    }
    if x > 0 {
        ::cvlr_asserts::cvlr_assume_checked(y > 0 && y < 10);
    }
}
pub fn main() {}
