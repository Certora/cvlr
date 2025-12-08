//! Tests for cvlr_assert_eq and related binary assertion macros

#[cfg(feature = "rt")]
extern crate cvlr;

#[cfg(feature = "rt")]
use cvlr_asserts::*;

#[cfg(feature = "rt")]
#[test]
fn test_assert_eq_pass() {
    cvlr_assert_eq!(1, 1);
    cvlr_assert_eq!(42, 42);
    cvlr_assert_eq!(0, 0);
    cvlr_assert_eq!(-1, -1);
    cvlr_assert_eq!(true, true);
    cvlr_assert_eq!(false, false);
    cvlr_assert_eq!("hello", "hello");
}

#[cfg(feature = "rt")]
#[test]
fn test_assert_eq_with_description() {
    cvlr_assert_eq!(1, 1, "values should be equal");
    cvlr_assert_eq!(42, 42, "test description");
}

#[cfg(feature = "rt")]
#[test]
fn test_assert_ne_pass() {
    cvlr_assert_ne!(1, 2);
    cvlr_assert_ne!(0, 1);
    cvlr_assert_ne!(true, false);
    cvlr_assert_ne!("hello", "world");
}

// #[cfg(feature = "rt")]
// #[test]
// #[should_panic]
// fn test_assert_ne_fail() {
//     cvlr_assert_ne!(1, 1);
// }

#[cfg(feature = "rt")]
#[test]
fn test_assert_ne_with_description() {
    cvlr_assert_ne!(1, 2, "values should not be equal");
}

#[cfg(feature = "rt")]
#[test]
fn test_assert_le_pass() {
    cvlr_assert_le!(1, 2);
    cvlr_assert_le!(1, 1);
    cvlr_assert_le!(0, 100);
    cvlr_assert_le!(-5, -1);
    cvlr_assert_le!(-10, 0);
}

#[cfg(feature = "rt")]
#[test]
fn test_assert_le_with_description() {
    cvlr_assert_le!(1, 2, "left should be less than or equal to right");
}

#[cfg(feature = "rt")]
#[test]
fn test_assert_lt_pass() {
    cvlr_assert_lt!(1, 2);
    cvlr_assert_lt!(0, 100);
    cvlr_assert_lt!(-5, -1);
    cvlr_assert_lt!(-10, 0);
}

#[cfg(feature = "rt")]
#[test]
fn test_assert_lt_with_description() {
    cvlr_assert_lt!(1, 2, "left should be less than right");
}

#[cfg(feature = "rt")]
#[test]
fn test_assert_ge_pass() {
    cvlr_assert_ge!(2, 1);
    cvlr_assert_ge!(1, 1);
    cvlr_assert_ge!(100, 0);
    cvlr_assert_ge!(-1, -5);
    cvlr_assert_ge!(0, -10);
}

#[cfg(feature = "rt")]
#[test]
fn test_assert_ge_with_description() {
    cvlr_assert_ge!(2, 1, "left should be greater than or equal to right");
}

#[cfg(feature = "rt")]
#[test]
fn test_assert_gt_pass() {
    cvlr_assert_gt!(2, 1);
    cvlr_assert_gt!(100, 0);
    cvlr_assert_gt!(-1, -5);
    cvlr_assert_gt!(0, -10);
}

#[cfg(feature = "rt")]
#[test]
fn test_assert_gt_with_description() {
    cvlr_assert_gt!(2, 1, "left should be greater than right");
}

// Test assume macros
#[cfg(feature = "rt")]
#[test]
fn test_assume_eq_pass() {
    cvlr_assume_eq!(1, 1);
    cvlr_assume_eq!(42, 42);
}

#[cfg(feature = "rt")]
#[test]
fn test_assume_eq_with_description() {
    cvlr_assume_eq!(1, 1, "assume values are equal");
}

#[cfg(feature = "rt")]
#[test]
fn test_assume_ne_pass() {
    cvlr_assume_ne!(1, 2);
}

#[cfg(feature = "rt")]
#[test]
fn test_assume_le_pass() {
    cvlr_assume_le!(1, 2);
    cvlr_assume_le!(1, 1);
}

#[cfg(feature = "rt")]
#[test]
fn test_assume_lt_pass() {
    cvlr_assume_lt!(1, 2);
}

#[cfg(feature = "rt")]
#[test]
fn test_assume_ge_pass() {
    cvlr_assume_ge!(2, 1);
    cvlr_assume_ge!(1, 1);
}

#[cfg(feature = "rt")]
#[test]
fn test_assume_gt_pass() {
    cvlr_assume_gt!(2, 1);
}

// Test with expressions
#[cfg(feature = "rt")]
#[test]
fn test_assert_eq_with_expressions() {
    let x = 5;
    let y = 3;
    cvlr_assert_eq!(x + y, 8);
    cvlr_assert_eq!(x * y, 15);
}

// Test with different numeric types
#[cfg(feature = "rt")]
#[test]
fn test_assert_eq_different_numeric_types() {
    cvlr_assert_eq!(1u8, 1u8);
    cvlr_assert_eq!(1u16, 1u16);
    cvlr_assert_eq!(1u32, 1u32);
    cvlr_assert_eq!(1u64, 1u64);
    cvlr_assert_eq!(1i8, 1i8);
    cvlr_assert_eq!(1i16, 1i16);
    cvlr_assert_eq!(1i32, 1i32);
    cvlr_assert_eq!(1i64, 1i64);
}
