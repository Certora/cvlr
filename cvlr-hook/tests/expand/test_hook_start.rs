use cvlr_hook::cvlr_hook_on_entry;

fn hook() {
    ();
}

// plain function instead of println! so the expanded output does not
// depend on the compiler's builtin macro lowering, which changes
// between rustc versions
fn work(_msg: &str) {
    ();
}

#[cvlr_hook_on_entry(hook())]
fn t1() {
    // hook inserted here
    work("t1");
}
