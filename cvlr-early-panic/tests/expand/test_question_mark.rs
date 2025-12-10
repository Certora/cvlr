use cvlr_early_panic::early_panic;

#[early_panic]
fn test_question_mark() -> Result<u64, String> {
    let v = "42".parse::<u64>()?;
    Ok(v)
}

#[early_panic]
fn test_nested_question_mark() -> Result<u64, String> {
    let v = "42".parse::<u64>()?.checked_add(1).ok_or("overflow")?;
    Ok(v)
}

#[early_panic]
fn test_multiple_question_marks() -> Result<u64, String> {
    let a = "10".parse::<u64>()?;
    let b = "20".parse::<u64>()?;
    Ok(a + b)
}

fn main() {}
