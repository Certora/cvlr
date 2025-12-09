//! Tests for cvlr_log macro

#[cfg(feature = "rt")]
extern crate cvlr_log;

#[cfg(feature = "rt")]
use cvlr_log::*;

#[test]
fn test_cvlr_log_macro_expansion() {
    macrotest::expand("tests/expand/*.rs");
}

#[cfg(feature = "rt")]
#[test]
fn test_cvlr_log_empty() {
    // Test empty macro call - should log location
    cvlr_log!();
}

#[cfg(feature = "rt")]
#[test]
fn test_cvlr_log_single_value_with_tag() {
    // Test single value with explicit tag
    cvlr_log!(42 => "answer");
    cvlr_log!(-10 => "negative");
    cvlr_log!(true => "boolean");
    cvlr_log!(false => "boolean_false");
    cvlr_log!("hello" => "greeting");
}

#[cfg(feature = "rt")]
#[test]
fn test_cvlr_log_single_value_without_tag() {
    // Test single value without tag (auto-generated from expression)
    let x = 42;
    cvlr_log!(x);
    cvlr_log!(100);
    cvlr_log!(true);
    cvlr_log!("world");
}

#[cfg(feature = "rt")]
#[test]
fn test_cvlr_log_with_logger() {
    // Test single value with explicit tag and logger
    let mut logger = CvlrLogger::new();
    cvlr_log!(42 => "value"; logger);
    cvlr_log!("test" => "string"; logger);
    cvlr_log!(true => "flag"; logger);
}

#[cfg(feature = "rt")]
#[test]
fn test_cvlr_log_multiple_values() {
    // Test multiple values
    let a = 1;
    let b = 2;
    let c = 3;
    cvlr_log!(a, b, c);
    cvlr_log!(10, 20, 30, 40);
    cvlr_log!("first", "second", "third");
}

#[cfg(feature = "rt")]
#[test]
fn test_cvlr_log_various_integer_types() {
    // Test various integer types
    cvlr_log!(1u8 => "u8");
    cvlr_log!(2u16 => "u16");
    cvlr_log!(3u32 => "u32");
    cvlr_log!(4u64 => "u64");
    cvlr_log!(5usize => "usize");
    cvlr_log!(6u128 => "u128");
    
    cvlr_log!(-1i8 => "i8");
    cvlr_log!(-2i16 => "i16");
    cvlr_log!(-3i32 => "i32");
    cvlr_log!(-4i64 => "i64");
    cvlr_log!(-5i128 => "i128");
}

#[cfg(feature = "rt")]
#[test]
fn test_cvlr_log_option() {
    // Test Option types
    let some_value: Option<u64> = Some(42);
    let none_value: Option<u64> = None;
    
    cvlr_log!(some_value => "some");
    cvlr_log!(none_value => "none");
    cvlr_log!(Some(100));
    cvlr_log!(None::<u32>);
}

#[cfg(feature = "rt")]
#[test]
fn test_cvlr_log_result() {
    // Test Result types
    let ok_value: Result<u64, &str> = Ok(42);
    let err_value: Result<u64, &str> = Err("error message");
    
    cvlr_log!(ok_value => "ok_result");
    cvlr_log!(err_value => "err_result");
    cvlr_log!(Ok::<u64, &str>(100));
    cvlr_log!(Err::<u64, &str>("failure"));
}

#[cfg(feature = "rt")]
#[test]
fn test_cvlr_log_unit() {
    // Test unit type
    cvlr_log!(() => "unit");
    let unit = ();
    cvlr_log!(unit);
}

#[cfg(feature = "rt")]
#[test]
fn test_cvlr_log_reference() {
    // Test references
    let value = 42;
    let ref_value = &value;
    cvlr_log!(ref_value => "reference");
    cvlr_log!(&100 => "literal_ref");
}

#[cfg(feature = "rt")]
#[test]
fn test_cvlr_log_complex_expressions() {
    // Test complex expressions
    cvlr_log!((1 + 2) => "sum");
    cvlr_log!((10 * 5) => "product");
    cvlr_log!((100 - 50) => "difference");
    
    let x = 5;
    let y = 10;
    cvlr_log!((x + y) => "computed");
}

#[cfg(feature = "rt")]
#[test]
fn test_cvlr_log_trailing_comma() {
    // Test trailing comma in multiple values
    cvlr_log!(1, 2, 3,);
    cvlr_log!("a", "b", "c",);
}

#[cfg(feature = "rt")]
#[test]
fn test_cvlr_log_mixed_types() {
    // Test logging multiple values of different types
    let num = 42;
    let text = "hello";
    let flag = true;
    
    cvlr_log!(num, text, flag);
}

#[cfg(feature = "rt")]
#[test]
fn test_cvlr_log_nested_options() {
    // Test nested Option types
    let nested: Option<Option<u64>> = Some(Some(42));
    cvlr_log!(nested => "nested_some");
    
    let nested_none: Option<Option<u64>> = Some(None);
    cvlr_log!(nested_none => "nested_none");
}

#[cfg(feature = "rt")]
#[test]
fn test_cvlr_log_nested_results() {
    // Test nested Result types
    let nested_ok: Result<Result<u64, &str>, &str> = Ok(Ok(42));
    cvlr_log!(nested_ok => "nested_ok");
    
    let nested_err: Result<Result<u64, &str>, &str> = Ok(Err("inner error"));
    cvlr_log!(nested_err => "nested_err");
}
