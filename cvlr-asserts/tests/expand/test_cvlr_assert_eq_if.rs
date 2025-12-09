use cvlr_asserts::cvlr_assert_eq_if;

fn main() {
    let x = 1;
    let flag = true;
    let a = 1;
    let b = 2;
    let x = 2;
    let y = 2;
    cvlr_assert_eq_if!(x > 0, a, b);
    cvlr_assert_eq_if!(flag, x, y, "if flag then x == y");
}
