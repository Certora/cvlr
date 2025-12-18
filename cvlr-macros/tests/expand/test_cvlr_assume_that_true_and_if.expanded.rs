use cvlr_macros::cvlr_assume_that;
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
        ::cvlr_asserts::cvlr_assume_checked(flag);
    } else {
        ()
    };
    if guard {
        ::cvlr_asserts::cvlr_assume_checked(x > 0);
    } else {
        ::cvlr_asserts::cvlr_assume_checked(y > 0);
    };
    if guard {
        ::cvlr_asserts::cvlr_assume_checked(a < b);
    } else {
        ::cvlr_asserts::cvlr_assume_checked(b > a);
    };
    if guard {
        if flag {
            ::cvlr_asserts::cvlr_assume_checked(x > 0);
        } else {
            ::cvlr_asserts::cvlr_assume_checked(y > 0);
        }
    } else {
        ()
    };
    if guard {
        ()
    } else {
        ::cvlr_asserts::cvlr_assume_checked(flag);
    };
    if guard {
        ::cvlr_asserts::cvlr_assume_checked(flag);
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
        ::cvlr_asserts::cvlr_assume_checked(flag);
    }
    if guard {
        ::cvlr_asserts::cvlr_assume_checked(x > 0);
    }
    if guard {
        ()
    }
}
pub fn main() {}
