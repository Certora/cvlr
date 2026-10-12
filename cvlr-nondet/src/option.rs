use crate::{nondet, Nondet};

impl<T: Nondet> Nondet for Option<T> {
    #[inline]
    fn nondet() -> Option<T> {
        if nondet::<bool>() {
            Some(nondet::<T>())
        } else {
            None
        }
    }
}

/// Either `None` or `Some(func())`, chosen arbitrarily.
///
/// `func` is called only on the `Some` path. Use this instead of
/// `nondet::<Option<T>>()` when `T` does not implement [`Nondet`], or when the
/// contained value must be built or constrained in a particular way:
///
/// ```ignore
/// let authority: Option<Pubkey> = nondet_option(|| cvlr_nondet_pubkey());
/// ```
pub fn nondet_option<T, F>(func: F) -> Option<T>
where
    F: FnOnce() -> T,
{
    if nondet::<bool>() {
        Some(func())
    } else {
        None
    }
}
