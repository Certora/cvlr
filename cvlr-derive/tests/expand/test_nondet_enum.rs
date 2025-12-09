use cvlr_derive::Nondet;

#[derive(Nondet)]
enum MyEnum {
    Variant1,
    Variant2(u64),
}

fn main() {}
