use cvlr_derive::Nondet;

#[derive(Nondet)]
struct Tuple(u64, i32, bool);

fn main() {
    let _ = Tuple::nondet();
}
