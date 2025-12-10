use cvlr_early_panic::early_panic;
fn test_return_err() -> Result<u64, u64> {
    ::core::panicking::panic("explicit panic");
}
fn test_return_err_conditional(x: u64) -> Result<u64, u64> {
    if x > 10 {
        ::core::panicking::panic("explicit panic");
    }
    Ok(x)
}
fn test_return_err_nested() -> Result<u64, u64> {
    if true {
        if false {
            ::core::panicking::panic("explicit panic");
        }
        ::core::panicking::panic("explicit panic");
    }
    Ok(0)
}
fn main() {}
