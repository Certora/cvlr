use cvlr_log::cvlr_log;
fn main() {
    let some_value: Option<u64> = Some(42);
    let none_value: Option<u64> = None;
    ::cvlr_log::cvlr_log("some", &(some_value));
    ::cvlr_log::cvlr_log("none", &(none_value));
    ::cvlr_log::cvlr_log("Some(100)", &(Some(100)));
    ::cvlr_log::cvlr_log("None::<u32>", &(None::<u32>));
}
