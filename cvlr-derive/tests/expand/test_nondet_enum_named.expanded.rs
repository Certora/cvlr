use cvlr_derive::Nondet;
enum MyEnum {
    Variant1,
    Variant2(u64),
    Variant3 { x: u64, y: i32 },
}
impl ::cvlr::nondet::Nondet for MyEnum {
    fn nondet() -> MyEnum {
        match ::cvlr::nondet::nondet::<u64>() {
            0u64 => MyEnum::Variant1,
            1u64 => MyEnum::Variant2(::cvlr::nondet::nondet()),
            _ => {
                MyEnum::Variant3 {
                    x: ::cvlr::nondet::nondet(),
                    y: ::cvlr::nondet::nondet(),
                }
            }
        }
    }
}
fn main() {
    let _ = MyEnum::nondet();
}
