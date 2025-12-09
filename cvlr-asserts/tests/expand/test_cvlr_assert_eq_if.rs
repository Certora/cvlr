use cvlr_asserts::cvlr_assert_eq_if;

fn main() {
    cvlr_assert_eq_if!(x > 0, a, b);
    cvlr_assert_eq_if!(flag, x, y, "if flag then x == y");
}
