use cvlr_macros::cvlr_eval_all;

pub fn test_eval_all() {
    let x = 5;
    let y = 10;
    let c = true;
    let flag = true;
    let a = 1;
    let b = 2;

    // Multiple unguarded expressions
    let _result1 = cvlr_eval_all!(x > 0, y < 20, x < y);

    // Mixed guarded and unguarded
    let _result2 = cvlr_eval_all!(x > 0, if c { x < y } else { true });

    // Using semicolons
    let _result3 = cvlr_eval_all!(x > 0; y < 20; if c { x < y } else { true });

    // Mixed separators
    let _result4 = cvlr_eval_all!(x > 0, y < 20; if c { x < y } else { true });

    // Boolean expressions
    let _result5 = cvlr_eval_all!(flag, x > 0 && y < 20);

    // Guarded boolean expressions
    let _result6 = cvlr_eval_all!(if flag { x > 0 } else { true }, if c { y < 20 } else { true });

    // Complex expressions
    let _result7 = cvlr_eval_all!(x + 1 > 0, y * 2 < 30, if a > 0 { b < 10 } else { true });
}

pub fn main() {
    test_eval_all();
}

