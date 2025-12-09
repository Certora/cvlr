use cvlr_derive::Nondet;
struct Tuple(u64, i32, bool);
impl ::cvlr::nondet::Nondet for Tuple {
    fn nondet() -> Tuple {
        Tuple(
            ::cvlr::nondet::nondet(),
            ::cvlr::nondet::nondet(),
            ::cvlr::nondet::nondet(),
        )
    }
}
fn main() {
    let _ = Tuple::nondet();
}
