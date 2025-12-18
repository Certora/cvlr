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

    // Literal true should expand to unit ()
    cvlr_assert_that!(true);

    // Unguarded boolean expressions
    cvlr_assert_that!(flag);
    cvlr_assert_that!(x > 0 && y < 10);
    cvlr_assert_that!(a || b);
    cvlr_assert_that!(!condition);
    cvlr_assert_that!(x + y > 0);

    // Group-wrapped boolean expressions
    cvlr_assert_that!((flag));
    cvlr_assert_that!((x > 0 && y < 10));
    cvlr_assert_that!(((a || b)));

    // Guarded boolean expressions
    cvlr_assert_that!(if guard { condition } else { true });
    cvlr_assert_that!(if x > 0 { y > 0 && z < 10 } else { true });
    cvlr_assert_that!(if flag { !error } else { true });
    cvlr_assert_that!(if test { (a || b) && c } else { true });
}

fn main() {}
