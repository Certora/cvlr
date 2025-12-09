use cvlr_derive::Nondet;

#[derive(Nondet)]
struct UnitStruct;

fn main() {
    let _ = UnitStruct::nondet();
}
