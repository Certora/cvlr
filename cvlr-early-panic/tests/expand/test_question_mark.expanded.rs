use cvlr_early_panic::early_panic;
fn test_question_mark() -> Result<u64, String> {
    let v = "42".parse::<u64>().unwrap();
    Ok(v)
}
fn test_nested_question_mark() -> Result<u64, String> {
    let v = "42".parse::<u64>().unwrap().checked_add(1).ok_or("overflow").unwrap();
    Ok(v)
}
fn test_multiple_question_marks() -> Result<u64, String> {
    let a = "10".parse::<u64>().unwrap();
    let b = "20".parse::<u64>().unwrap();
    Ok(a + b)
}
fn main() {}
