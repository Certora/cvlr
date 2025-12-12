use cvlr_macros::cvlr_eval_that;

pub fn test_eval_that() {
    let a = 1;
    let b = 2;
    let x = 3;
    let y = 4;
    let flag = true;

    // Unguarded comparisons
    let _result1 = cvlr_eval_that!(a < b);
    let _result2 = cvlr_eval_that!(x <= y);
    let _result3 = cvlr_eval_that!(x > 0);
    let _result4 = cvlr_eval_that!(x >= 0);
    let _result5 = cvlr_eval_that!(x == 3);
    let _result6 = cvlr_eval_that!(x != 0);

    // Unguarded boolean expressions
    let _result7 = cvlr_eval_that!(flag);
    let _result8 = cvlr_eval_that!(x > 0 && y < 10);

    // Guarded comparisons
    let _result9 = cvlr_eval_that!(if flag { a < b });
    let _result10 = cvlr_eval_that!(if x > 0 { y <= 10 });

    // Guarded boolean expressions
    let _result11 = cvlr_eval_that!(if flag { x > 0 });
    let _result12 = cvlr_eval_that!(if x > 0 { y > 0 && y < 10 });

    // Complex expressions
    let _result13 = cvlr_eval_that!(x + 1 < y * 2);
    let _result14 = cvlr_eval_that!(if a > 0 { b < 10 });
}

pub fn main() {
    test_eval_that();
}

