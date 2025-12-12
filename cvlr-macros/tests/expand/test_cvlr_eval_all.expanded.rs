use cvlr_macros::cvlr_eval_all;
pub fn test_eval_all() {
    let x = 5;
    let y = 10;
    let c = true;
    let flag = true;
    let a = 1;
    let b = 2;
    let _result1 = {
        let __cvlr_eval_all_res = true;
        let __cvlr_eval_all_res = __cvlr_eval_all_res && { x > 0 };
        let __cvlr_eval_all_res = __cvlr_eval_all_res && { y < 20 };
        let __cvlr_eval_all_res = __cvlr_eval_all_res && { x < y };
        __cvlr_eval_all_res
    };
    let _result2 = {
        let __cvlr_eval_all_res = true;
        let __cvlr_eval_all_res = __cvlr_eval_all_res && { x > 0 };
        let __cvlr_eval_all_res = __cvlr_eval_all_res
            && { if c { x < y } else { true } };
        __cvlr_eval_all_res
    };
    let _result3 = {
        let __cvlr_eval_all_res = true;
        let __cvlr_eval_all_res = __cvlr_eval_all_res && { x > 0 };
        let __cvlr_eval_all_res = __cvlr_eval_all_res && { y < 20 };
        let __cvlr_eval_all_res = __cvlr_eval_all_res
            && { if c { x < y } else { true } };
        __cvlr_eval_all_res
    };
    let _result4 = {
        let __cvlr_eval_all_res = true;
        let __cvlr_eval_all_res = __cvlr_eval_all_res && { x > 0 };
        let __cvlr_eval_all_res = __cvlr_eval_all_res && { y < 20 };
        let __cvlr_eval_all_res = __cvlr_eval_all_res
            && { if c { x < y } else { true } };
        __cvlr_eval_all_res
    };
    let _result5 = {
        let __cvlr_eval_all_res = true;
        let __cvlr_eval_all_res = __cvlr_eval_all_res && { flag };
        let __cvlr_eval_all_res = __cvlr_eval_all_res && { x > 0 && y < 20 };
        __cvlr_eval_all_res
    };
    let _result6 = {
        let __cvlr_eval_all_res = true;
        let __cvlr_eval_all_res = __cvlr_eval_all_res
            && { if flag { x > 0 } else { true } };
        let __cvlr_eval_all_res = __cvlr_eval_all_res
            && { if c { y < 20 } else { true } };
        __cvlr_eval_all_res
    };
    let _result7 = {
        let __cvlr_eval_all_res = true;
        let __cvlr_eval_all_res = __cvlr_eval_all_res && { x + 1 > 0 };
        let __cvlr_eval_all_res = __cvlr_eval_all_res && { y * 2 < 30 };
        let __cvlr_eval_all_res = __cvlr_eval_all_res
            && { if a > 0 { b < 10 } else { true } };
        __cvlr_eval_all_res
    };
}
pub fn main() {
    test_eval_all();
}
