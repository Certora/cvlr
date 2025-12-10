use cvlr_early_panic::early_panic;

#[early_panic]
fn test_bare_err() -> Result<u64, u64> {
    Err(42)
}

#[early_panic]
fn test_bare_err_conditional(x: u64) -> Result<u64, u64> {
    if x > 10 {
        Err(42)
    } else {
        Ok(x)
    }
}

#[early_panic]
fn test_bare_err_with_question_mark() -> Result<u64, String> {
    let v = "42".parse::<u64>()?;
    if v > 100 {
        Err("too large".to_string())
    } else {
        Ok(v)
    }
}

fn main() {}
