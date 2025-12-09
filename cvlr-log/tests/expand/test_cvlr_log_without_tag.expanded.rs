use cvlr_log::cvlr_log;
fn main() {
    let x = 42;
    ::cvlr_log::cvlr_log("x", &(x));
    ::cvlr_log::cvlr_log("100", &(100));
    ::cvlr_log::cvlr_log("true", &(true));
    ::cvlr_log::cvlr_log("\"world\"", &("world"));
}
