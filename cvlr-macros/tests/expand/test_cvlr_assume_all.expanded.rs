use cvlr_macros::cvlr_assume_all;
pub fn test_assume_all_comma_separated() {
    let x = 5;
    let y = 10;
    let z = 15;
    let a = 1;
    let b = 2;
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assume x > 0"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs > __cvlr_rhs);
    };
    {
        let __cvlr_lhs = y;
        let __cvlr_rhs = 20;
        ::cvlr_log::cvlr_log("_", &("assume y < 20"));
        ::cvlr_log::cvlr_log("y", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("20", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
    };
    {
        let __cvlr_lhs = z;
        let __cvlr_rhs = x;
        ::cvlr_log::cvlr_log("_", &("assume z > x"));
        ::cvlr_log::cvlr_log("z", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("x", &(__cvlr_rhs));
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
        let __cvlr_rhs = 5;
        ::cvlr_log::cvlr_log("_", &("assume x == 5"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("5", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs == __cvlr_rhs);
    };
    {
        let __cvlr_lhs = y;
        let __cvlr_rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assume y != 0"));
        ::cvlr_log::cvlr_log("y", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs != __cvlr_rhs);
    };
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assume x > 0"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs > __cvlr_rhs);
    };
    {
        let __cvlr_lhs = y;
        let __cvlr_rhs = 20;
        ::cvlr_log::cvlr_log("_", &("assume y < 20"));
        ::cvlr_log::cvlr_log("y", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("20", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
    };
    {
        let __cvlr_lhs = z;
        let __cvlr_rhs = x;
        ::cvlr_log::cvlr_log("_", &("assume z > x"));
        ::cvlr_log::cvlr_log("z", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("x", &(__cvlr_rhs));
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
        let __cvlr_rhs = 5;
        ::cvlr_log::cvlr_log("_", &("assume x == 5"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("5", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs == __cvlr_rhs);
    };
    {
        let __cvlr_lhs = y;
        let __cvlr_rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assume y != 0"));
        ::cvlr_log::cvlr_log("y", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs != __cvlr_rhs);
    };
}
pub fn test_assume_all_semicolon_separated() {
    let x = 5;
    let y = 10;
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assume x > 0"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs > __cvlr_rhs);
    };
    {
        let __cvlr_lhs = y;
        let __cvlr_rhs = 20;
        ::cvlr_log::cvlr_log("_", &("assume y < 20"));
        ::cvlr_log::cvlr_log("y", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("20", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
    };
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = y;
        ::cvlr_log::cvlr_log("_", &("assume x < y"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("y", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
    };
}
pub fn test_assume_all_mixed_separators() {
    let x = 5;
    let y = 10;
    let flag = true;
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assume x > 0"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs > __cvlr_rhs);
    };
    {
        let __cvlr_lhs = y;
        let __cvlr_rhs = 20;
        ::cvlr_log::cvlr_log("_", &("assume y < 20"));
        ::cvlr_log::cvlr_log("y", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("20", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
    };
    if flag {
        {
            let __cvlr_lhs = x;
            let __cvlr_rhs = y;
            ::cvlr_log::cvlr_log("_", &("assume x < y"));
            ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("y", &(__cvlr_rhs));
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
        };
    }
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assume x > 0"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs > __cvlr_rhs);
    };
    {
        let __cvlr_lhs = y;
        let __cvlr_rhs = 20;
        ::cvlr_log::cvlr_log("_", &("assume y < 20"));
        ::cvlr_log::cvlr_log("y", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("20", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
    };
    if flag {
        {
            let __cvlr_lhs = x;
            let __cvlr_rhs = y;
            ::cvlr_log::cvlr_log("_", &("assume x < y"));
            ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("y", &(__cvlr_rhs));
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
        };
    }
}
pub fn test_assume_all_guarded() {
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
            ::cvlr_log::cvlr_log("_", &("assume y < 20"));
            ::cvlr_log::cvlr_log("y", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("20", &(__cvlr_rhs));
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
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
            ::cvlr_log::cvlr_log("_", &("assume y < 20"));
            ::cvlr_log::cvlr_log("y", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("20", &(__cvlr_rhs));
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
        };
    }
}
pub fn test_assume_all_mixed_guarded_unguarded() {
    let x = 5;
    let y = 10;
    let flag = true;
    let a = 1;
    let b = 2;
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assume x > 0"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs > __cvlr_rhs);
    };
    if flag {
        {
            let __cvlr_lhs = x;
            let __cvlr_rhs = y;
            ::cvlr_log::cvlr_log("_", &("assume x < y"));
            ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("y", &(__cvlr_rhs));
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
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
    {
        let __cvlr_lhs = y;
        let __cvlr_rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assume y > 0"));
        ::cvlr_log::cvlr_log("y", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs > __cvlr_rhs);
    };
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assume x > 0"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs > __cvlr_rhs);
    };
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
    {
        let __cvlr_lhs = y;
        let __cvlr_rhs = 20;
        ::cvlr_log::cvlr_log("_", &("assume y < 20"));
        ::cvlr_log::cvlr_log("y", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("20", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
    };
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assume x > 0"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs > __cvlr_rhs);
    };
    if flag {
        {
            let __cvlr_lhs = x;
            let __cvlr_rhs = y;
            ::cvlr_log::cvlr_log("_", &("assume x < y"));
            ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
            ::cvlr_log::cvlr_log("y", &(__cvlr_rhs));
            ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
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
    {
        let __cvlr_lhs = y;
        let __cvlr_rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assume y > 0"));
        ::cvlr_log::cvlr_log("y", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs > __cvlr_rhs);
    };
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assume x > 0"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("0", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs > __cvlr_rhs);
    };
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
    {
        let __cvlr_lhs = y;
        let __cvlr_rhs = 20;
        ::cvlr_log::cvlr_log("_", &("assume y < 20"));
        ::cvlr_log::cvlr_log("y", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("20", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs < __cvlr_rhs);
    };
}
pub fn main() {}
