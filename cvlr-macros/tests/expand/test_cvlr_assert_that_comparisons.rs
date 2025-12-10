use cvlr_macros::cvlr_assert_that;

pub fn test_comparisons() {
    let a = 1;
    let b = 2;
    let x = 3;
    let y = 4;
    let p = 5;
    let q = 6;
    let m = 7;
    let n = 8;
    let c = 9;

    // Unguarded comparisons - all operators
    cvlr_assert_that!(a < b);
    cvlr_assert_that!(x <= y);
    cvlr_assert_that!(p > q);
    cvlr_assert_that!(m >= n);
    cvlr_assert_that!(x == y);
    cvlr_assert_that!(a != b);

    // With expressions
    cvlr_assert_that!(x + 1 < y * 2);
    cvlr_assert_that!(a > c);
}

pub fn main() {}
