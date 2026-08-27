use cvlr_hook::{cvlr_hook_on_entry, cvlr_hook_on_exit};
fn hook_start() {
    ();
}
fn hook_end() {
    ();
}
fn check_eq(_a: i32, _b: i32) {
    ();
}
fn check_res(_a: Result<()>, _b: Result<()>) {
    ();
}
fn log(_msg: &str) {
    ();
}
fn tmp() -> Result<()> {
    hook_start();
    hook_end();
    Ok(())
}
fn t3() {
    check_res(tmp(), Ok(()));
}
fn t4() {
    hook_start();
    check_eq(1, 1);
    hook_end();
}
fn abs(x: i32) -> i32 {
    hook_start();
    if x >= 0 {
        log("x is positive");
        x
    } else {
        log("x is negative");
        -x
    }
}
fn abs2(x: i32) -> i32 {
    hook_end();
    if x >= 0 {
        log("x is positive");
        x
    } else {
        log("x is negative");
        -x
    }
}
