use cvlr_macros::cvlr_assume_all;
pub fn test_assume_all_comma_separated() {
    let x = 5;
    let y = 10;
    let z = 15;
    let a = 1;
    let b = 2;
    {
        let lhs = x;
        let rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assume x > 0"));
        ::cvlr_log::cvlr_log("x", &(lhs));
        ::cvlr_log::cvlr_log("0", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs > rhs);
    };
    {
        let lhs = y;
        let rhs = 20;
        ::cvlr_log::cvlr_log("_", &("assume y < 20"));
        ::cvlr_log::cvlr_log("y", &(lhs));
        ::cvlr_log::cvlr_log("20", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs < rhs);
    };
    {
        let lhs = z;
        let rhs = x;
        ::cvlr_log::cvlr_log("_", &("assume z > x"));
        ::cvlr_log::cvlr_log("z", &(lhs));
        ::cvlr_log::cvlr_log("x", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs > rhs);
    };
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
        let rhs = 5;
        ::cvlr_log::cvlr_log("_", &("assume x == 5"));
        ::cvlr_log::cvlr_log("x", &(lhs));
        ::cvlr_log::cvlr_log("5", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs == rhs);
    };
    {
        let lhs = y;
        let rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assume y != 0"));
        ::cvlr_log::cvlr_log("y", &(lhs));
        ::cvlr_log::cvlr_log("0", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs != rhs);
    };
    {
        let lhs = x;
        let rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assume x > 0"));
        ::cvlr_log::cvlr_log("x", &(lhs));
        ::cvlr_log::cvlr_log("0", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs > rhs);
    };
    {
        let lhs = y;
        let rhs = 20;
        ::cvlr_log::cvlr_log("_", &("assume y < 20"));
        ::cvlr_log::cvlr_log("y", &(lhs));
        ::cvlr_log::cvlr_log("20", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs < rhs);
    };
    {
        let lhs = z;
        let rhs = x;
        ::cvlr_log::cvlr_log("_", &("assume z > x"));
        ::cvlr_log::cvlr_log("z", &(lhs));
        ::cvlr_log::cvlr_log("x", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs > rhs);
    };
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
        let rhs = 5;
        ::cvlr_log::cvlr_log("_", &("assume x == 5"));
        ::cvlr_log::cvlr_log("x", &(lhs));
        ::cvlr_log::cvlr_log("5", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs == rhs);
    };
    {
        let lhs = y;
        let rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assume y != 0"));
        ::cvlr_log::cvlr_log("y", &(lhs));
        ::cvlr_log::cvlr_log("0", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs != rhs);
    };
}
pub fn test_assume_all_semicolon_separated() {
    let x = 5;
    let y = 10;
    {
        let lhs = x;
        let rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assume x > 0"));
        ::cvlr_log::cvlr_log("x", &(lhs));
        ::cvlr_log::cvlr_log("0", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs > rhs);
    };
    {
        let lhs = y;
        let rhs = 20;
        ::cvlr_log::cvlr_log("_", &("assume y < 20"));
        ::cvlr_log::cvlr_log("y", &(lhs));
        ::cvlr_log::cvlr_log("20", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs < rhs);
    };
    {
        let lhs = x;
        let rhs = y;
        ::cvlr_log::cvlr_log("_", &("assume x < y"));
        ::cvlr_log::cvlr_log("x", &(lhs));
        ::cvlr_log::cvlr_log("y", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs < rhs);
    };
}
pub fn test_assume_all_mixed_separators() {
    let x = 5;
    let y = 10;
    let flag = true;
    {
        let lhs = x;
        let rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assume x > 0"));
        ::cvlr_log::cvlr_log("x", &(lhs));
        ::cvlr_log::cvlr_log("0", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs > rhs);
    };
    {
        let lhs = y;
        let rhs = 20;
        ::cvlr_log::cvlr_log("_", &("assume y < 20"));
        ::cvlr_log::cvlr_log("y", &(lhs));
        ::cvlr_log::cvlr_log("20", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs < rhs);
    };
    if flag {
        {
            let lhs = x;
            let rhs = y;
            ::cvlr_log::cvlr_log("_", &("assume x < y"));
            ::cvlr_log::cvlr_log("x", &(lhs));
            ::cvlr_log::cvlr_log("y", &(rhs));
            ::cvlr_asserts::cvlr_assume_checked(lhs < rhs);
        };
    }
    {
        let lhs = x;
        let rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assume x > 0"));
        ::cvlr_log::cvlr_log("x", &(lhs));
        ::cvlr_log::cvlr_log("0", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs > rhs);
    };
    {
        let lhs = y;
        let rhs = 20;
        ::cvlr_log::cvlr_log("_", &("assume y < 20"));
        ::cvlr_log::cvlr_log("y", &(lhs));
        ::cvlr_log::cvlr_log("20", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs < rhs);
    };
    if flag {
        {
            let lhs = x;
            let rhs = y;
            ::cvlr_log::cvlr_log("_", &("assume x < y"));
            ::cvlr_log::cvlr_log("x", &(lhs));
            ::cvlr_log::cvlr_log("y", &(rhs));
            ::cvlr_asserts::cvlr_assume_checked(lhs < rhs);
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
            ::cvlr_log::cvlr_log("_", &("assume y < 20"));
            ::cvlr_log::cvlr_log("y", &(lhs));
            ::cvlr_log::cvlr_log("20", &(rhs));
            ::cvlr_asserts::cvlr_assume_checked(lhs < rhs);
        };
    }
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
            ::cvlr_log::cvlr_log("_", &("assume y < 20"));
            ::cvlr_log::cvlr_log("y", &(lhs));
            ::cvlr_log::cvlr_log("20", &(rhs));
            ::cvlr_asserts::cvlr_assume_checked(lhs < rhs);
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
        let lhs = x;
        let rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assume x > 0"));
        ::cvlr_log::cvlr_log("x", &(lhs));
        ::cvlr_log::cvlr_log("0", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs > rhs);
    };
    if flag {
        {
            let lhs = x;
            let rhs = y;
            ::cvlr_log::cvlr_log("_", &("assume x < y"));
            ::cvlr_log::cvlr_log("x", &(lhs));
            ::cvlr_log::cvlr_log("y", &(rhs));
            ::cvlr_asserts::cvlr_assume_checked(lhs < rhs);
        };
    }
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
    {
        let lhs = y;
        let rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assume y > 0"));
        ::cvlr_log::cvlr_log("y", &(lhs));
        ::cvlr_log::cvlr_log("0", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs > rhs);
    };
    {
        let lhs = x;
        let rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assume x > 0"));
        ::cvlr_log::cvlr_log("x", &(lhs));
        ::cvlr_log::cvlr_log("0", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs > rhs);
    };
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
    {
        let lhs = y;
        let rhs = 20;
        ::cvlr_log::cvlr_log("_", &("assume y < 20"));
        ::cvlr_log::cvlr_log("y", &(lhs));
        ::cvlr_log::cvlr_log("20", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs < rhs);
    };
    {
        let lhs = x;
        let rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assume x > 0"));
        ::cvlr_log::cvlr_log("x", &(lhs));
        ::cvlr_log::cvlr_log("0", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs > rhs);
    };
    if flag {
        {
            let lhs = x;
            let rhs = y;
            ::cvlr_log::cvlr_log("_", &("assume x < y"));
            ::cvlr_log::cvlr_log("x", &(lhs));
            ::cvlr_log::cvlr_log("y", &(rhs));
            ::cvlr_asserts::cvlr_assume_checked(lhs < rhs);
        };
    }
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
    {
        let lhs = y;
        let rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assume y > 0"));
        ::cvlr_log::cvlr_log("y", &(lhs));
        ::cvlr_log::cvlr_log("0", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs > rhs);
    };
    {
        let lhs = x;
        let rhs = 0;
        ::cvlr_log::cvlr_log("_", &("assume x > 0"));
        ::cvlr_log::cvlr_log("x", &(lhs));
        ::cvlr_log::cvlr_log("0", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs > rhs);
    };
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
    {
        let lhs = y;
        let rhs = 20;
        ::cvlr_log::cvlr_log("_", &("assume y < 20"));
        ::cvlr_log::cvlr_log("y", &(lhs));
        ::cvlr_log::cvlr_log("20", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs < rhs);
    };
}
pub fn main() {}
