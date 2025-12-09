use cvlr_log::cvlr_log;

fn main() {
    let a = 1;
    let b = 2;
    let c = 3;
    cvlr_log!(a, b, c);
    cvlr_log!(10, 20, 30, 40);
    cvlr_log!("first", "second", "third");
}
