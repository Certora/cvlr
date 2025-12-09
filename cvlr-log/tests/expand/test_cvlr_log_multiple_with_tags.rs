use cvlr_log::cvlr_log;

fn main() {
    let a = 1;
    let b = 2;
    let c = 3;
    cvlr_log!(a => "first", b => "second", c => "third");
    cvlr_log!(10 => "ten", 20 => "twenty", 30 => "thirty", 40 => "forty");
    cvlr_log!("hello" => "greeting", "world" => "target");
    cvlr_log!(a => "a", b => "b",); // with trailing comma
}
