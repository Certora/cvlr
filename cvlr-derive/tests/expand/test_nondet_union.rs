use cvlr_derive::Nondet;

#[derive(Nondet)]
union MyUnion {
    f1: u32,
    f2: i32,
}

fn main() {}
