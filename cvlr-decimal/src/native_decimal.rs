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
}

pub trait AsDecimal<const D: u32> {
    fn as_decimal(&self) -> NativeDecimal<D>;
}

impl<const D: u32, T> AsDecimal<D> for T
where
    T: Into<NativeInt> + Copy,
{
    fn as_decimal(&self) -> NativeDecimal<D> {
        NativeDecimal::new((*self).into())
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
        Self::new(nondet::<NativeInt>())
    }
}

impl<const D: u32> core::ops::Add<NativeDecimal<D>> for NativeDecimal<D> {
    type Output = Self;
    fn add(self, other: NativeDecimal<D>) -> Self::Output {
        Self::new(self.val + other.val)
    }
}

impl<const D: u32, T> core::ops::Add<T> for NativeDecimal<D>
where
    T: Into<NativeInt>,
{
    type Output = Self;
    fn add(self, other: T) -> Self::Output {
        Self::new(self.as_int() + other.into())
    }
}

impl<const D: u32> core::ops::Add<NativeDecimal<D>> for NativeInt {
    type Output = NativeDecimal<D>;
    fn add(self, other: NativeDecimal<D>) -> Self::Output {
        Self::Output::new(self + other.as_int())
    }
}

impl<const D: u32, T> core::ops::Mul<T> for NativeDecimal<D>
where
    T: Into<NativeInt>,
{
    type Output = Self;
    fn mul(self, other: T) -> Self::Output {
        Self::new(self.as_int() * other.into())
    }
}

impl<const D: u32> core::ops::Mul<NativeDecimal<D>> for NativeInt {
    type Output = NativeDecimal<D>;
    fn mul(self, other: NativeDecimal<D>) -> Self::Output {
        Self::Output::new(self * other.as_int())
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
}
