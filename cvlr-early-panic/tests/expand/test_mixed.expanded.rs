use cvlr_early_panic::early_panic;
fn test_mixed_question_mark_and_return_err() -> Result<u64, String> {
    let v = "42".parse::<u64>().unwrap();
    if v > 100 {
        ::core::panicking::panic("explicit panic");
    }
    Ok(v)
}
fn test_mixed_all_patterns() -> Result<u64, String> {
    let v = "42".parse::<u64>().unwrap();
    if v == 0 {
        ::core::panicking::panic("explicit panic");
    }
    if v > 100 {
        ::core::panicking::panic("explicit panic");
    } else {
        Ok(v)
    }
}
fn test_complex_nested() -> Result<u64, String> {
    let a = "10".parse::<u64>().unwrap();
    if a > 5 {
        let b = "20".parse::<u64>().unwrap();
        if b > 15 {
            ::core::panicking::panic("explicit panic");
        }
        Ok(b)
    } else {
        ::core::panicking::panic("explicit panic");
    }
}
fn main() {}
