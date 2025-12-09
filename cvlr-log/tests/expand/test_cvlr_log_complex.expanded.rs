use cvlr_log::cvlr_log;
fn main() {
    ::cvlr_log::cvlr_log("sum", &((1 + 2)));
    ::cvlr_log::cvlr_log("product", &((10 * 5)));
    let x = 5;
    let y = 10;
    ::cvlr_log::cvlr_log("computed", &((x + y)));
    ::cvlr_log::cvlr_log("x", &(x));
    ::cvlr_log::cvlr_log("y", &(y));
}
