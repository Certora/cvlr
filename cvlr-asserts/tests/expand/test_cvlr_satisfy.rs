use cvlr_asserts::cvlr_satisfy;

fn main() {
    cvlr_satisfy!(true);
    cvlr_satisfy!(x == y);
    cvlr_satisfy!(z > 0, "z must be positive");
}
