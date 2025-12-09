use cvlr_derive::Nondet;
struct Point {
    x: u64,
    y: u64,
}
impl ::cvlr::nondet::Nondet for Point {
    fn nondet() -> Point {
        Point {
            x: ::cvlr::nondet::nondet(),
            y: ::cvlr::nondet::nondet(),
        }
    }
}
fn main() {
    let _ = Point::nondet();
}
