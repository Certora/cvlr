use cvlr_log::cvlr_log;

fn main() {
    let some_value: Option<u64> = Some(42);
    let none_value: Option<u64> = None;
    
    cvlr_log!(some_value => "some");
    cvlr_log!(none_value => "none");
    cvlr_log!(Some(100));
    cvlr_log!(None::<u32>);
}
