use cvlr_derive::Nondet;
enum MyEnum {
    Variant1,
    Variant2(u64),
}
impl ::cvlr::nondet::Nondet for MyEnum {
    fn nondet() -> MyEnum {
        match ::cvlr::nondet::nondet::<u64>() {
            0u64 => MyEnum::Variant1,
            _ => MyEnum::Variant2(::cvlr::nondet::nondet()),
        }
    }
}
fn main() {}
