use cvlr_log::{CvlrLog, CvlrLogger};
use cvlr_mathint::NativeInt;
use cvlr_nondet::{nondet, Nondet};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct NativeDecimal<const D: u32> {
    val: NativeInt,
}

impl<const D: u32> NativeDecimal<D> {
    pub fn new(val: NativeInt) -> Self {
        Self { val }
    }

    pub fn as_int(&self) -> NativeInt {
        self.val
    }

    pub fn as_decimal<T: Into<NativeInt>>(v: T) -> Self {
        Self::new(v.into())
    }
}

pub trait AsDecimal<const D: u32> {
    fn as_decimal(&self) -> NativeDecimal<D>;
}

impl<const D: u32, T> AsDecimal<D> for T
where
    T: Into<NativeInt> + Copy,
{
    fn as_decimal(&self) -> NativeDecimal<D> {
        NativeDecimal::as_decimal((*self).into())
    }
}

impl<const D: u32> CvlrLog for NativeDecimal<D> {
    fn log(&self, tag: &str, logger: &mut CvlrLogger) {
        logger.log_u64_as_dec(tag, self.val.into(), D as u64);
    }
}

impl<const D: u32> core::ops::Deref for NativeDecimal<D> {
    type Target = NativeInt;
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl<const D: u32> Nondet for NativeDecimal<D> {
    fn nondet() -> Self {
        Self::as_decimal(nondet::<NativeInt>())
    }
}

impl<const D: u32> core::ops::Add<NativeDecimal<D>> for NativeDecimal<D> {
    type Output = Self;
    fn add(self, other: NativeDecimal<D>) -> Self::Output {
        Self::as_decimal(self.val + other.val)
    }
}

impl<const D: u32, T> core::ops::Add<T> for NativeDecimal<D>
where
    T: Into<NativeInt>,
{
    type Output = Self;
    fn add(self, other: T) -> Self::Output {
        Self::as_decimal(self.as_int() + other.into())
    }
}

impl<const D: u32> core::ops::Add<NativeDecimal<D>> for NativeInt {
    type Output = NativeDecimal<D>;
    fn add(self, other: NativeDecimal<D>) -> Self::Output {
        Self::Output::as_decimal(self + other.as_int())
    }
}

impl<const D: u32, T> core::ops::Mul<T> for NativeDecimal<D>
where
    T: Into<NativeInt>,
{
    type Output = Self;
    fn mul(self, other: T) -> Self::Output {
        Self::as_decimal(self.as_int() * other.into())
    }
}

impl<const D: u32> core::ops::Mul<NativeDecimal<D>> for NativeInt {
    type Output = NativeDecimal<D>;
    fn mul(self, other: NativeDecimal<D>) -> Self::Output {
        Self::Output::as_decimal(self * other.as_int())
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use super::*;
    use alloc::vec;

    #[test]
    fn test_new_and_as_int() {
        let val: NativeInt = 42.into();
        let decimal: NativeDecimal<2> = NativeDecimal::new(val);
        assert_eq!(decimal.as_int(), val);
    }

    #[test]
    fn test_as_decimal_trait() {
        // Test with u64
        let val: u64 = 100;
        let decimal: NativeDecimal<2> = val.as_decimal();
        assert_eq!(decimal.as_int(), val);

        // Test with u32
        let val: u32 = 50;
        let decimal: NativeDecimal<4> = val.as_decimal();
        assert_eq!(decimal.as_int(), val);

        // Test with u8
        let val: u8 = 10;
        let decimal: NativeDecimal<6> = val.as_decimal();
        assert_eq!(decimal.as_int(), val);

        // Test with i32
        let val: i32 = 5;
        let decimal: NativeDecimal<2> = val.as_decimal();
        assert_eq!(decimal.as_int(), val);
    }

    #[test]
    fn test_addition_native_decimal() {
        let a: NativeDecimal<2> = NativeDecimal::new(10.into());
        let b: NativeDecimal<2> = NativeDecimal::new(20.into());
        let result = a + b;
        assert_eq!(result.as_int(), 30);
    }

    #[test]
    fn test_addition_with_primitive() {
        let a: NativeDecimal<2> = NativeDecimal::new(10.into());
        let result = a + 5u64;
        assert_eq!(result.as_int(), 15);

        let result = a + 3u32;
        assert_eq!(result.as_int(), 13);
    }

    #[test]
    fn test_addition_native_int_with_decimal() {
        let a: NativeInt = 5.into();
        let b: NativeDecimal<2> = NativeDecimal::new(10.into());
        let result: NativeDecimal<2> = a + b;
        assert_eq!(result.as_int(), 15);
    }

    #[test]
    fn test_multiplication_with_primitive() {
        let a: NativeDecimal<2> = NativeDecimal::new(10.into());
        let result = a * 3u64;
        assert_eq!(result.as_int(), 30);

        let result = a * 2u32;
        assert_eq!(result.as_int(), 20);
    }

    #[test]
    fn test_multiplication_native_int_with_decimal() {
        let a: NativeInt = 5.into();
        let b: NativeDecimal<2> = NativeDecimal::new(10.into());
        let result: NativeDecimal<2> = a * b;
        assert_eq!(result.as_int(), 50);
    }

    #[test]
    fn test_comparison_operations() {
        let a: NativeDecimal<2> = NativeDecimal::new(5.into());
        let b: NativeDecimal<2> = NativeDecimal::new(10.into());
        let c: NativeDecimal<2> = NativeDecimal::new(5.into());

        // Equality
        assert_eq!(a, c);
        assert_ne!(a, b);

        // Less than
        assert!(a < b);
        assert!(!(b < a));
        assert!(!(a < c));

        // Less than or equal
        assert!(a <= b);
        assert!(a <= c);
        assert!(!(b <= a));

        // Greater than
        assert!(b > a);
        assert!(!(a > b));
        assert!(!(a > c));

        // Greater than or equal
        assert!(b >= a);
        assert!(a >= c);
        assert!(!(a >= b));
    }

    #[test]
    fn test_ordering() {
        let values = vec![
            NativeDecimal::<2>::new(3.into()),
            NativeDecimal::<2>::new(1.into()),
            NativeDecimal::<2>::new(4.into()),
            NativeDecimal::<2>::new(2.into()),
        ];
        let mut sorted = values.clone();
        sorted.sort();
        assert_eq!(sorted[0].as_int(), 1);
        assert_eq!(sorted[1].as_int(), 2);
        assert_eq!(sorted[2].as_int(), 3);
        assert_eq!(sorted[3].as_int(), 4);
    }

    #[test]
    fn test_clone_and_copy() {
        let a: NativeDecimal<2> = NativeDecimal::new(42.into());
        let b = a; // Copy
        let c = a.clone(); // Clone
        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!(b, c);
    }

    #[test]
    fn test_deref() {
        let decimal: NativeDecimal<2> = NativeDecimal::new(100.into());
        let native_int: &NativeInt = &*decimal;
        assert_eq!(*native_int, 100);
    }

    #[test]
    fn test_different_precision_constants() {
        let val: u64 = 100;
        let decimal_2: NativeDecimal<2> = val.as_decimal();
        let decimal_4: NativeDecimal<4> = val.as_decimal();
        let decimal_6: NativeDecimal<6> = val.as_decimal();

        // All should have the same underlying value
        assert_eq!(decimal_2.as_int(), decimal_4.as_int());
        assert_eq!(decimal_4.as_int(), decimal_6.as_int());

        // But they are different types, so they can't be directly compared
        // However, we can compare their underlying values
        assert_eq!(decimal_2.as_int(), decimal_6.as_int());
    }

    #[test]
    fn test_chained_operations() {
        let a: NativeDecimal<2> = NativeDecimal::new(10.into());
        let b: NativeDecimal<2> = NativeDecimal::new(20.into());
        let result = a + b + 5u64;
        assert_eq!(result.as_int(), 35);

        let result = a * 2u64 + b;
        assert_eq!(result.as_int(), 40);
    }

    #[test]
    fn test_as_decimal_static_method() {
        // Test with u8
        let val: u8 = 42;
        let decimal: NativeDecimal<2> = NativeDecimal::as_decimal(val);
        let expected: NativeInt = val.into();
        assert_eq!(decimal.as_int(), expected);

        // Test with u16
        let val: u16 = 1000;
        let decimal: NativeDecimal<3> = NativeDecimal::as_decimal(val);
        let expected: NativeInt = val.into();
        assert_eq!(decimal.as_int(), expected);

        // Test with u32
        let val: u32 = 50000;
        let decimal: NativeDecimal<4> = NativeDecimal::as_decimal(val);
        let expected: NativeInt = val.into();
        assert_eq!(decimal.as_int(), expected);

        // Test with u64
        let val: u64 = 1000000;
        let decimal: NativeDecimal<5> = NativeDecimal::as_decimal(val);
        let expected: NativeInt = val.into();
        assert_eq!(decimal.as_int(), expected);

        // Test with u128
        let val: u128 = 999999999;
        let decimal: NativeDecimal<6> = NativeDecimal::as_decimal(val);
        let expected: NativeInt = val.into();
        assert_eq!(decimal.as_int(), expected);

        // Test with i32 (positive)
        let val: i32 = 42;
        let decimal: NativeDecimal<2> = NativeDecimal::as_decimal(val);
        let expected: NativeInt = val.into();
        assert_eq!(decimal.as_int(), expected);

        // Test with i32 (zero)
        let val: i32 = 0;
        let decimal: NativeDecimal<2> = NativeDecimal::as_decimal(val);
        let expected: NativeInt = val.into();
        assert_eq!(decimal.as_int(), expected);

        // Test with NativeInt directly
        let val: NativeInt = 12345.into();
        let decimal: NativeDecimal<7> = NativeDecimal::as_decimal(val);
        assert_eq!(decimal.as_int(), val);

        // Test with different precision constants
        let val: u64 = 100;
        let decimal_2: NativeDecimal<2> = NativeDecimal::as_decimal(val);
        let decimal_4: NativeDecimal<4> = NativeDecimal::as_decimal(val);
        let decimal_8: NativeDecimal<8> = NativeDecimal::as_decimal(val);

        // All should have the same underlying value
        let expected: NativeInt = val.into();
        assert_eq!(decimal_2.as_int(), expected);
        assert_eq!(decimal_4.as_int(), expected);
        assert_eq!(decimal_8.as_int(), expected);
    }

    #[test]
    fn test_as_decimal_static_method_edge_cases() {
        // Test with zero
        let decimal: NativeDecimal<2> = NativeDecimal::as_decimal(0u64);
        let expected: NativeInt = 0u64.into();
        assert_eq!(decimal.as_int(), expected);

        // Test with maximum u8
        let decimal: NativeDecimal<2> = NativeDecimal::as_decimal(u8::MAX);
        let expected: NativeInt = u8::MAX.into();
        assert_eq!(decimal.as_int(), expected);

        // Test with maximum u16
        let decimal: NativeDecimal<2> = NativeDecimal::as_decimal(u16::MAX);
        let expected: NativeInt = u16::MAX.into();
        assert_eq!(decimal.as_int(), expected);

        // Test with maximum u32
        let decimal: NativeDecimal<2> = NativeDecimal::as_decimal(u32::MAX);
        let expected: NativeInt = u32::MAX.into();
        assert_eq!(decimal.as_int(), expected);

        // Test with large u64
        let val: u64 = 18446744073709551615; // u64::MAX
        let decimal: NativeDecimal<2> = NativeDecimal::as_decimal(val);
        let expected: NativeInt = val.into();
        assert_eq!(decimal.as_int(), expected);
    }

    #[test]
    fn test_as_decimal_static_method_consistency() {
        // Test that as_decimal produces the same result as new
        let val: NativeInt = 42.into();
        let decimal_new: NativeDecimal<2> = NativeDecimal::new(val);
        let decimal_as_decimal: NativeDecimal<2> = NativeDecimal::as_decimal(val);
        assert_eq!(decimal_new, decimal_as_decimal);
        assert_eq!(decimal_new.as_int(), decimal_as_decimal.as_int());

        // Test with different types but same value
        let val_u64: u64 = 100;
        let val_u32: u32 = 100;
        let val_u16: u16 = 100;
        let val_u8: u8 = 100;

        let decimal_u64: NativeDecimal<2> = NativeDecimal::as_decimal(val_u64);
        let decimal_u32: NativeDecimal<2> = NativeDecimal::as_decimal(val_u32);
        let decimal_u16: NativeDecimal<2> = NativeDecimal::as_decimal(val_u16);
        let decimal_u8: NativeDecimal<2> = NativeDecimal::as_decimal(val_u8);

        let expected: NativeInt = val_u64.into();
        assert_eq!(decimal_u64.as_int(), expected);
        assert_eq!(decimal_u32.as_int(), expected);
        assert_eq!(decimal_u16.as_int(), expected);
        assert_eq!(decimal_u8.as_int(), expected);
    }
}
