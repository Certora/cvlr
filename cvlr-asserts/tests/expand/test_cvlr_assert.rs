use cvlr_asserts::cvlr_assert;

fn main() {
    cvlr_assert!(true);
    cvlr_assert!(1 == 1);
    cvlr_assert!(false, "this should fail");
    cvlr_assert!(x > 0, "x must be positive");
}
