use cvlr_asserts::cvlr_assume_eq;
fn main() {
    {
        let lhs = 1;
        let rhs = 1;
        ::cvlr_log::cvlr_log("_", &("assume 1 == 1"));
        ::cvlr_log::cvlr_log("1", &(lhs));
        ::cvlr_log::cvlr_log("1", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs == rhs);
    };
    {
        let lhs = x;
        let rhs = y;
        ::cvlr_log::cvlr_log("_", &("assume x == y"));
        ::cvlr_log::cvlr_log("x", &(lhs));
        ::cvlr_log::cvlr_log("y", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs == rhs);
    };
    {
        let lhs = a;
        let rhs = b;
        ::cvlr_log::cvlr_log("_", &("assume a == b"));
        ::cvlr_log::cvlr_log("a", &(lhs));
        ::cvlr_log::cvlr_log("b", &(rhs));
        ::cvlr_asserts::cvlr_assume_checked(lhs == rhs);
    };
}
