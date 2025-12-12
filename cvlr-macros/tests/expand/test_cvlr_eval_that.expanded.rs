use cvlr_macros::cvlr_eval_that;
pub fn test_eval_that() {
    let a = 1;
    let b = 2;
    let x = 3;
    let y = 4;
    let flag = true;
    let _result1 = { a < b };
    let _result2 = { x <= y };
    let _result3 = { x > 0 };
    let _result4 = { x >= 0 };
    let _result5 = { x == 3 };
    let _result6 = { x != 0 };
    let _result7 = { flag };
    let _result8 = { x > 0 && y < 10 };
    let _result9 = { if flag { a < b } else { true } };
    let _result10 = { if x > 0 { y <= 10 } else { true } };
    let _result11 = { if flag { x > 0 } else { true } };
    let _result12 = { if x > 0 { y > 0 && y < 10 } else { true } };
    let _result13 = { x + 1 < y * 2 };
    let _result14 = { if a > 0 { b < 10 } else { true } };
}
pub fn main() {
    test_eval_that();
}
