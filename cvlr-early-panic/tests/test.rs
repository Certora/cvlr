use cvlr_early_panic::early_panic;
use std::str::FromStr;

#[early_panic]
fn test_one_payload() -> Result<u64, <u64 as FromStr>::Err> {
    let v = "fail42".parse::<u64>()?;
    Ok(v)
}

#[test]
#[should_panic]
fn test_one() {
    let _ = test_one_payload();
}

#[early_panic]
fn test_two_payload() -> Result<u64, u64> {
    Err(42)
}

#[test]
#[should_panic]
fn test_two() {
    let _ = test_two_payload();
}

#[early_panic]
fn test_three_payload(a: u64) -> Result<u64, u64> {
    if a > 10 {
        return Err(42);
    }
    Ok(a)
}

#[test]
#[should_panic]
fn test_three() {
    let _ = test_three_payload(11);
}
