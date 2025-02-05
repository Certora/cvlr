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
