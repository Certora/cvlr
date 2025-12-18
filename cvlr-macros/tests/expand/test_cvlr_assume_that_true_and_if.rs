use cvlr_macros::cvlr_assume_that;

pub fn test_true_literal() {
    // Literal true should expand to unit ()
    cvlr_assume_that!(true);
}

pub fn test_if_else_expressions() {
    let guard = true;
    let flag = false;
    let x = 5;
    let y = 10;
    let a = 1;
    let b = 2;

    // If-else with boolean expressions
    cvlr_assume_that!(if guard { flag } else { true });
    cvlr_assume_that!(if guard { x > 0 } else { y > 0 });
    cvlr_assume_that!(if guard { a < b } else { b > a });

    // Nested if-else
    cvlr_assume_that!(if guard { if flag { x > 0 } else { y > 0 } } else { true });

    // If-else with true in branches
    cvlr_assume_that!(if guard { true } else { flag });
    cvlr_assume_that!(if guard { flag } else { true });
    cvlr_assume_that!(if guard { true } else { true });
}

pub fn test_if_without_else() {
    let guard = true;
    let flag = false;
    let x = 5;

    // If without else branch
    cvlr_assume_that!(if guard { flag });
    cvlr_assume_that!(if guard { x > 0 });
    cvlr_assume_that!(if guard { true });
}

pub fn main() {}

