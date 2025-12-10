use cvlr_early_panic::early_panic;
fn test_option_question_mark() -> Option<u64> {
    let v = Some(42).unwrap();
    Some(v)
}
fn test_option_none() -> Option<u64> {
    None
}
fn test_option_return_none() -> Option<u64> {
    return None;
}
fn main() {}
