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
    cvlr_assert_that!(if flag { a < b } else { true });
    cvlr_assert_that!(if x > 0 { y <= z } else { true });
    cvlr_assert_that!(if cond { p > q } else { true });
    cvlr_assert_that!(if guard { m >= n } else { true });
    cvlr_assert_that!(if test { x == y } else { true });
    cvlr_assert_that!(if check { a != b } else { true });

    // Complex guards and conditions
    cvlr_assert_that!(if a > c { d < p } else { true });
    cvlr_assert_that!(if x + 1 > 0 { y * 2 < z } else { true });
}

pub fn main() {}
