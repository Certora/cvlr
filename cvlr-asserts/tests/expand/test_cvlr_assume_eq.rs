use cvlr_asserts::cvlr_assume_eq;

fn main() {
    cvlr_assume_eq!(1, 1);
    cvlr_assume_eq!(x, y);
    cvlr_assume_eq!(a, b, "assume a equals b");
}
