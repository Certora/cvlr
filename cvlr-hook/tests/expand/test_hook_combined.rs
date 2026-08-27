use cvlr_hook::{cvlr_hook_on_entry, cvlr_hook_on_exit};

fn hook_start() {
    ();
}

fn hook_end() {
    ();
}

// plain functions instead of assert_eq!/println! so the expanded output
// does not depend on the compiler's builtin macro lowering, which
// changes between rustc versions
fn check_eq(_a: i32, _b: i32) {
    ();
}

fn check_res(_a: Result<()>, _b: Result<()>) {
    ();
}

fn log(_msg: &str) {
    ();
}

#[cvlr_hook_on_entry(hook_start())]
#[cvlr_hook_on_exit(hook_end())]
fn tmp() -> Result<()> {
    // hook start inserted here
    // hook end inserted here
    Ok(())
}

fn t3() {
    check_res(tmp(), Ok(()));
}

#[cvlr_hook_on_entry(hook_start())]
#[cvlr_hook_on_exit(hook_end())]
fn t4() {
    // hook start inserted here
    check_eq(1, 1);
    // hook end inserted here
}

#[cvlr_hook_on_entry(hook_start())]
fn abs(x : i32) -> i32 {
    // hook start inserted here
    if x >= 0 {
        log("x is positive");
        x
    } else {
        log("x is negative");
        -x
    }
}

#[cvlr_hook_on_exit(hook_end())]
fn abs2(x : i32) -> i32 {
    // hook end inserted here
    if x >= 0 {
        log("x is positive");
        x
    } else {
        log("x is negative");
        -x
    }
}
