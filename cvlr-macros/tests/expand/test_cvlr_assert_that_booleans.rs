use cvlr_macros::cvlr_assert_that;

pub fn test() {
    let flag = true;
    let x = 5;
    let y = 3;
    let a = true;
    let b = false;
    let condition = false;
    let guard = true;
    let z = 7;
    let error = false;
    let test = true;
    let c = true;

    // Unguarded boolean expressions
    cvlr_assert_that!(flag);
    cvlr_assert_that!(x > 0 && y < 10);
    cvlr_assert_that!(a || b);
    cvlr_assert_that!(!condition);
    cvlr_assert_that!(x + y > 0);

    // Guarded boolean expressions
    cvlr_assert_that!(guard => condition);
    cvlr_assert_that!(x > 0 => y > 0 && z < 10);
    cvlr_assert_that!(flag => !error);
    cvlr_assert_that!(test => (a || b) && c);
}

fn main() {}
