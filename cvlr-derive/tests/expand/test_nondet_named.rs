use cvlr_derive::Nondet;

#[derive(Nondet)]
struct Point {
    x: u64,
    y: u64,
}

fn main() {
    let _ = Point::nondet();
}
