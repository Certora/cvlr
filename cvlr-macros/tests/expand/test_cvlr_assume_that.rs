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

    // Unguarded comparisons - all operators
    cvlr_assume_that!(a < b);
    cvlr_assume_that!(x <= y);
    cvlr_assume_that!(p > q);
    cvlr_assume_that!(m >= n);
    cvlr_assume_that!(x == y);
    cvlr_assume_that!(a != b);

    // With expressions
    cvlr_assume_that!(x + 1 < y * 2);
    cvlr_assume_that!(a > c);

    // Group-wrapped comparisons
    cvlr_assume_that!((a < b));
    cvlr_assume_that!((x > y));
    cvlr_assume_that!((p <= q));
    cvlr_assume_that!((m >= n));
    cvlr_assume_that!((x == y));
    cvlr_assume_that!((a != b));

    // Nested groups
    cvlr_assume_that!(((a < b)));
    cvlr_assume_that!((((x > y))));
}

pub fn test_assume_guarded_comparisons() {
    let flag = true;
    let a = 1;
    let b = 2;
    let x = 5;
    let y = 10;

    // Guarded comparisons
    cvlr_assume_that!(if flag { a < b } else { true });
    cvlr_assume_that!(if x > 0 { y <= 20 } else { true });

    // Guarded comparisons with groups
    cvlr_assume_that!(if flag { a < b } else { true });
    cvlr_assume_that!(if x > 0 { y <= 20 } else { true });
    cvlr_assume_that!(if flag { a < b } else { true });
}

pub fn test_assume_booleans() {
    let flag = true;
    let x = 5;
    let y = 3;

    // Literal true should expand to unit ()
    cvlr_assume_that!(true);

    // Unguarded boolean expressions
    cvlr_assume_that!(flag);
    cvlr_assume_that!(x > 0 && y < 10);

    // Guarded boolean expressions
    cvlr_assume_that!(if flag { x > 0 } else { true });
    cvlr_assume_that!(if x > 0 { y > 0 && y < 10 } else { true });
}

pub fn main() {}
