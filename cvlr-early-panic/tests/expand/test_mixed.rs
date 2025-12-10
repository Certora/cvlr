use cvlr_early_panic::early_panic;

#[early_panic]
fn test_mixed_question_mark_and_return_err() -> Result<u64, String> {
    let v = "42".parse::<u64>()?;
    if v > 100 {
        return Err("too large".to_string());
    }
    Ok(v)
}

#[early_panic]
fn test_mixed_all_patterns() -> Result<u64, String> {
    let v = "42".parse::<u64>()?;
    if v == 0 {
        return Err("zero".to_string());
    }
    if v > 100 {
        Err("too large".to_string())
    } else {
        Ok(v)
    }
}

#[early_panic]
fn test_complex_nested() -> Result<u64, String> {
    let a = "10".parse::<u64>()?;
    if a > 5 {
        let b = "20".parse::<u64>()?;
        if b > 15 {
            return Err("both too large".to_string());
        }
        Ok(b)
    } else {
        Err("a too small".to_string())
    }
}

fn main() {}
