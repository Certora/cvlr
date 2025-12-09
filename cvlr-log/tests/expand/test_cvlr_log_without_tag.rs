use cvlr_log::cvlr_log;

fn main() {
    let x = 42;
    cvlr_log!(x);
    cvlr_log!(100);
    cvlr_log!(true);
    cvlr_log!("world");
}
