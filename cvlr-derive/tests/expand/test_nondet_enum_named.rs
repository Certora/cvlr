use cvlr_derive::Nondet;

#[derive(Nondet)]
enum MyEnum {
    Variant1,
    Variant2(u64),
    Variant3 { x: u64, y: i32 },
}

fn main() {
    let _ = MyEnum::nondet();
}
