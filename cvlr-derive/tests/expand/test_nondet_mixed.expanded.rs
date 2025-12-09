use cvlr_derive::Nondet;
struct Mixed {
    a: u8,
    b: i16,
    c: u32,
    d: i64,
    e: bool,
}
impl ::cvlr::nondet::Nondet for Mixed {
    fn nondet() -> Mixed {
        Mixed {
            a: ::cvlr::nondet::nondet(),
            b: ::cvlr::nondet::nondet(),
            c: ::cvlr::nondet::nondet(),
            d: ::cvlr::nondet::nondet(),
            e: ::cvlr::nondet::nondet(),
        }
    }
}
fn main() {
    let _ = Mixed::nondet();
}
