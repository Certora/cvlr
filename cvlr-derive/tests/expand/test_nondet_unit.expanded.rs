use cvlr_derive::Nondet;
struct UnitStruct;
impl ::cvlr::nondet::Nondet for UnitStruct {
    fn nondet() -> UnitStruct {
        UnitStruct
    }
}
fn main() {
    let _ = UnitStruct::nondet();
}
