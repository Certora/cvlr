use cvlr_asserts::cvlr_assume;
fn main() {
    ::cvlr_asserts::cvlr_assume_checked(true);
    ::cvlr_asserts::cvlr_assume_checked(x > 0);
    ::cvlr_asserts::cvlr_assume_checked(y < 100);
}
