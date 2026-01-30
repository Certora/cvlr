use cvlr_spec::cvlr_predicate;

struct Ctx {
    x: i32,
}

fn main() {
    let _ = cvlr_predicate! { | c : Ctx | -> {
        let threshold = 0;
        c.x > threshold;
    } };
}
