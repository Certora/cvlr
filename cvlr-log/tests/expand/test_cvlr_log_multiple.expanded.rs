use cvlr_log::cvlr_log;
fn main() {
    let a = 1;
    let b = 2;
    let c = 3;
    ::cvlr_log::cvlr_log("a", &(a));
    ::cvlr_log::cvlr_log("b", &(b));
    ::cvlr_log::cvlr_log("c", &(c));
    ::cvlr_log::cvlr_log("10", &(10));
    ::cvlr_log::cvlr_log("20", &(20));
    ::cvlr_log::cvlr_log("30", &(30));
    ::cvlr_log::cvlr_log("40", &(40));
    ::cvlr_log::cvlr_log("\"first\"", &("first"));
    ::cvlr_log::cvlr_log("\"second\"", &("second"));
    ::cvlr_log::cvlr_log("\"third\"", &("third"));
}
