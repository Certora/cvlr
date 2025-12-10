use cvlr_early_panic::early_panic;

#[early_panic]
fn test_return_err() -> Result<u64, u64> {
    return Err(42);
}

#[early_panic]
fn test_return_err_conditional(x: u64) -> Result<u64, u64> {
    if x > 10 {
        return Err(42);
    }
    Ok(x)
}

#[early_panic]
fn test_return_err_nested() -> Result<u64, u64> {
    if true {
        if false {
            return Err(1);
        }
        return Err(2);
    }
    Ok(0)
}

fn main() {}
