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
