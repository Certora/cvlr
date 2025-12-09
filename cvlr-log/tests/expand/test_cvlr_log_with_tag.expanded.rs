use cvlr_log::cvlr_log;
fn main() {
    ::cvlr_log::cvlr_log("answer", &(42));
    ::cvlr_log::cvlr_log("negative", &(-10));
    ::cvlr_log::cvlr_log("boolean", &(true));
    ::cvlr_log::cvlr_log("greeting", &("hello"));
}
