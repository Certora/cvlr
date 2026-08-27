use cvlr_hook::cvlr_hook_on_exit;
fn hook() {
    ();
}
fn check_eq(_a: i32, _b: i32) {
    ();
}
fn check_res(_a: Result<()>, _b: Result<()>) {
    ();
}
fn t1() {
    check_eq(1, 1);
    check_eq(2, 2);
    hook();
}
fn t2() {
    check_eq(1, 1);
    hook();
}
fn tmp() -> Result<()> {
    hook();
    Ok(())
}
fn t3() {
    check_res(tmp(), Ok(()));
}
