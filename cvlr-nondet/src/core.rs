
/// A trait for giving a type a non-deterministic value
pub trait Nondet: Sized {
    fn nondet() -> Self;

    fn nondet_with<F>(func: F) -> Self
    where
        F: FnOnce(&Self) -> bool,
    {
        let val = Self::nondet();
        cvlr_asserts::cvlr_assume!(func(&val));
        val
    }
}

/// Return a nondet value of type according to the Nondet trait
pub fn nondet<T: Nondet>() -> T {
    Nondet::nondet()
}

pub fn nondet_with<T: Nondet, F>(func: F) -> T
where
    F: FnOnce(&T) -> bool,
{
    Nondet::nondet_with(func)
}


#[macro_export]
macro_rules! nondet_impl {
    ($t:ty, $v:expr, $doc:tt) => {
        impl Nondet for $t {
            #[inline]
            #[doc = $doc]
            fn nondet() -> $t {
                $v
            }
        }
    };
}