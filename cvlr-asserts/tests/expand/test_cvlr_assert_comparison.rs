use cvlr_asserts::{cvlr_assert_le, cvlr_assert_lt, cvlr_assert_ge, cvlr_assert_gt};

fn main() {
    cvlr_assert_le!(1, 2);
    cvlr_assert_lt!(1, 2);
    cvlr_assert_ge!(2, 1);
    cvlr_assert_gt!(2, 1);
    cvlr_assert_le!(x, y, "x must be <= y");
}
