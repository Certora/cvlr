use cvlr_asserts::cvlr_assert_if;

fn main() {
    cvlr_assert_if!(x > 0, y > 0);
    cvlr_assert_if!(flag, value == expected);
}
