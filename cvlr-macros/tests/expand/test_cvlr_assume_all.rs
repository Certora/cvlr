use cvlr_macros::cvlr_assume_all;

pub fn test_assume_all_comma_separated() {
    let x = 5;
    let y = 10;
    let z = 15;
    let a = 1;
    let b = 2;

    // Multiple unguarded assumptions with commas
    cvlr_assume_all!(x > 0, y < 20, z > x);
    cvlr_assume_all!(a < b, x == 5, y != 0);

    // Group-wrapped comparisons
    cvlr_assume_all!((x > 0), (y < 20), (z > x));
    cvlr_assume_all!((a < b), ((x == 5)), (y != 0));
}

pub fn test_assume_all_semicolon_separated() {
    let x = 5;
    let y = 10;

    // Multiple assumptions with semicolons
    cvlr_assume_all!(x > 0; y < 20; x < y);
}

pub fn test_assume_all_mixed_separators() {
    let x = 5;
    let y = 10;
    let flag = true;

    // Mixed separators
    cvlr_assume_all!(x > 0, y < 20; if flag { x < y });
    cvlr_assume_all!(x > 0; y < 20, if flag { x < y });
}

pub fn test_assume_all_guarded() {
    let flag = true;
    let a = 1;
    let b = 2;
    let x = 5;
    let y = 10;

    // Multiple guarded assumptions
    cvlr_assume_all!(if flag { a < b }, if x > 0 { y < 20 });
    cvlr_assume_all!(if flag { a < b }; if x > 0 { y < 20 });
}

pub fn test_assume_all_mixed_guarded_unguarded() {
    let x = 5;
    let y = 10;
    let flag = true;
    let a = 1;
    let b = 2;

    // Mixed guarded and unguarded
    cvlr_assume_all!(x > 0, if flag { x < y });
    cvlr_assume_all!(if flag { a < b }, y > 0);
    cvlr_assume_all!(x > 0, if flag { a < b }, y < 20);

    // Mixed with group-wrapped expressions
    cvlr_assume_all!((x > 0), if flag { (x < y) });
    cvlr_assume_all!(if flag { (a < b) }, (y > 0));
    cvlr_assume_all!((x > 0), if flag { ((a < b)) }, (y < 20));
}

pub fn main() {}
