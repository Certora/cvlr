use cvlr_hook::cvlr_hook_on_exit;

fn hook() {
    ();
}

// plain functions instead of assert_eq! so the expanded output does not
// depend on the compiler's builtin macro lowering, which changes
// between rustc versions
fn check_eq(_a: i32, _b: i32) {
    ();
}

fn check_res(_a: Result<()>, _b: Result<()>) {
    ();
}

#[cvlr_hook_on_exit(hook())]
fn t1() {
    check_eq(1, 1);
    // hook inserted here
    check_eq(2, 2);
}

#[cvlr_hook_on_exit(hook())]
fn t2() {
    // hook inserted here
    check_eq(1, 1);
}

#[cvlr_hook_on_exit(hook())]
fn tmp() -> Result<()> {
    // hook inserted here
    Ok(())
}

fn t3() {
    check_res(tmp(), Ok(()));
}
