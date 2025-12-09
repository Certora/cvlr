use cvlr_asserts::cvlr_assert_ne;

fn main() {
    cvlr_assert_ne!(1, 2);
    cvlr_assert_ne!(x, y);
    cvlr_assert_ne!(a, b, "a and b must not be equal");
}
