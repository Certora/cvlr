use cvlr_log::cvlr_log;

fn main() {
    cvlr_log!(42 => "answer");
    cvlr_log!(-10 => "negative");
    cvlr_log!(true => "boolean");
    cvlr_log!("hello" => "greeting");
}
