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

    // Guarded comparisons - all operators
    cvlr_assert_that!(flag => a < b);
    cvlr_assert_that!(x > 0 => y <= z);
    cvlr_assert_that!(cond => p > q);
    cvlr_assert_that!(guard => m >= n);
    cvlr_assert_that!(test => x == y);
    cvlr_assert_that!(check => a != b);

    // Complex guards and conditions
    cvlr_assert_that!(a > c => d < p);
    cvlr_assert_that!(x + 1 > 0 => y * 2 < z);
}

pub fn main() {}
