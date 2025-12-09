use cvlr_asserts::cvlr_assume;

fn main() {
    cvlr_assume!(true);
    cvlr_assume!(x > 0);
    cvlr_assume!(y < 100, "y must be less than 100");
}
