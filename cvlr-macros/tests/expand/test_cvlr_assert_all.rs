use cvlr_macros::cvlr_assert_all;

pub fn test_assert_all_comma_separated() {
    let x = 5;
    let y = 10;
    let z = 15;
    let a = 1;
    let b = 2;

    // Multiple unguarded comparisons with commas
    cvlr_assert_all!(x > 0, y < 20, z > x);
    cvlr_assert_all!(a < b, x == 5, y != 0);

    // Group-wrapped comparisons
    cvlr_assert_all!((x > 0), (y < 20), (z > x));
    cvlr_assert_all!((a < b), ((x == 5)), (y != 0));
}

pub fn test_assert_all_semicolon_separated() {
    let x = 5;
    let y = 10;

    // Multiple assertions with semicolons
    cvlr_assert_all!(x > 0; y < 20; x < y);
}

pub fn test_assert_all_mixed_separators() {
    let x = 5;
    let y = 10;
    let flag = true;
    let c = 3;

    // Mixed separators
    cvlr_assert_all!(x > 0, y < 20; if flag { x < y } else { true });
    cvlr_assert_all!(x > 0; y < 20, if flag { c < y } else { true });
}

pub fn test_assert_all_guarded() {
    let flag = true;
    let a = 1;
    let b = 2;
    let x = 5;
    let y = 10;

    // Multiple guarded assertions
    cvlr_assert_all!(if flag { a < b } else { true }, if x > 0 { y < 20 } else { true });
    cvlr_assert_all!(if flag { a < b } else { true }; if x > 0 { y < 20 } else { true });
}

pub fn test_assert_all_mixed_guarded_unguarded() {
    let x = 5;
    let y = 10;
    let flag = true;
    let a = 1;
    let b = 2;

    // Mixed guarded and unguarded
    cvlr_assert_all!(x > 0, if flag { x < y } else { true });
    cvlr_assert_all!(if flag { a < b } else { true }, y > 0);
    cvlr_assert_all!(x > 0, if flag { a < b } else { true }, y < 20);
}

pub fn test_assert_all_boolean_expressions() {
    let flag = true;
    let x = 5;
    let y = 3;
    let z = 7;

    // Boolean expressions
    cvlr_assert_all!(flag, x > 0 && y < 10);
    cvlr_assert_all!(if flag { x > 0 } else { true }, if x > 0 { y > 0 && z < 10 } else { true });

    // Group-wrapped boolean expressions
    cvlr_assert_all!((flag), (x > 0 && y < 10));
    cvlr_assert_all!((((flag))), ((x > 0 && y < 10)));
}

pub fn main() {}
