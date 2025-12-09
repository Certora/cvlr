use cvlr_derive::Nondet;

#[derive(Nondet)]
struct Mixed {
    a: u8,
    b: i16,
    c: u32,
    d: i64,
    e: bool,
}

fn main() {
    let _ = Mixed::nondet();
}
