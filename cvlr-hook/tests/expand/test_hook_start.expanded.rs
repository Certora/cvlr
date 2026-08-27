use cvlr_hook::cvlr_hook_on_entry;
fn hook() {
    ();
}
fn work(_msg: &str) {
    ();
}
fn t1() {
    hook();
    work("t1");
}
