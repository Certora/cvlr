use cvlr_early_panic::early_panic;
fn test_bare_err() -> Result<u64, u64> {
    ::core::panicking::panic("explicit panic");
}
fn test_bare_err_conditional(x: u64) -> Result<u64, u64> {
    if x > 10 {
        ::core::panicking::panic("explicit panic");
    } else {
        Ok(x)
    }
}
fn test_bare_err_with_question_mark() -> Result<u64, String> {
    let v = "42".parse::<u64>().unwrap();
    if v > 100 {
        ::core::panicking::panic("explicit panic");
    } else {
        Ok(v)
    }
}
fn main() {}
