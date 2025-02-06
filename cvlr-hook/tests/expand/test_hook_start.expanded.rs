use cvlr_hook::cvlr_hook_on_entry;
fn hook() {
    ();
}
fn t1() {
    hook();
    {
        ::std::io::_print(format_args!("t1\n"));
    };
}
