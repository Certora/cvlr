use cvlr_asserts::cvlr_assume_eq;
fn main() {
    {
        let __cvlr_lhs = 1;
        let __cvlr_rhs = 1;
        ::cvlr_log::cvlr_log("_", &("assume 1 == 1"));
        ::cvlr_log::cvlr_log("1", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("1", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs == __cvlr_rhs);
    };
    {
        let __cvlr_lhs = x;
        let __cvlr_rhs = y;
        ::cvlr_log::cvlr_log("_", &("assume x == y"));
        ::cvlr_log::cvlr_log("x", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("y", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs == __cvlr_rhs);
    };
    {
        let __cvlr_lhs = a;
        let __cvlr_rhs = b;
        ::cvlr_log::cvlr_log("_", &("assume a == b"));
        ::cvlr_log::cvlr_log("a", &(__cvlr_lhs));
        ::cvlr_log::cvlr_log("b", &(__cvlr_rhs));
        ::cvlr_asserts::cvlr_assume_checked(__cvlr_lhs == __cvlr_rhs);
    };
}
