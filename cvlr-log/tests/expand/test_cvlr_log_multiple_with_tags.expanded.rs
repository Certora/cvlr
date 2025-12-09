use cvlr_log::cvlr_log;
fn main() {
    let a = 1;
    let b = 2;
    let c = 3;
    ::cvlr_log::cvlr_log("first", &(a));
    ::cvlr_log::cvlr_log("second", &(b));
    ::cvlr_log::cvlr_log("third", &(c));
    ::cvlr_log::cvlr_log("ten", &(10));
    ::cvlr_log::cvlr_log("twenty", &(20));
    ::cvlr_log::cvlr_log("thirty", &(30));
    ::cvlr_log::cvlr_log("forty", &(40));
    ::cvlr_log::cvlr_log("greeting", &("hello"));
    ::cvlr_log::cvlr_log("target", &("world"));
    ::cvlr_log::cvlr_log("a", &(a));
    ::cvlr_log::cvlr_log("b", &(b));
}
