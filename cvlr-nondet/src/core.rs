/// A trait for giving a type a non-deterministic value
///
/// A nondeterministic value is one the Prover may choose freely: a rule that
/// draws `let x: u64 = nondet();` is verified for every possible `u64`, and a
/// counterexample reports the value that breaks it. Nondeterministic values are
/// how rules model unknown inputs and program state.
///
/// Implemented for `()`, `bool`, all integer types from `u8`/`i8` to
/// `u128`/`i128` and `usize` (any value of the type), `Option<T>` (`None` or
/// `Some` of a nondeterministic `T`), and CVLR's numeric types (`NativeInt`,
/// `NativeFixedU64`, `NativeDecimal`, ...). Derive it for your own structs and
/// enums with `#[derive(Nondet)]`, or implement it with
/// [`nondet_impl!`](crate::nondet_impl). Each call returns a new value,
/// unrelated to any earlier one.
pub trait Nondet: Sized {
    /// Returns an arbitrary value of this type.
    fn nondet() -> Self;

    /// Returns an arbitrary value of this type for which `func` returns
    /// `true`; see the free function [`nondet_with`](crate::nondet_with).
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
///
/// The type is usually inferred: `let amount: u64 = nondet();`. Constrain the
/// value afterwards with `cvlr_assume!`, or draw it with
/// [`nondet_with`](crate::nondet_with). Also available in `cvlr`'s prelude as
/// `nondet` and `cvlr_nondet`.
pub fn nondet<T: Nondet>() -> T {
    Nondet::nondet()
}

/// An arbitrary value of type `T` that satisfies `func`.
///
/// Draws `nondet::<T>()` and then assumes (`cvlr_assume!`) that `func` returns
/// `true` for it, so only executions where the predicate holds are analyzed.
/// If no value of `T` satisfies `func`, every later assertion passes
/// vacuously.
///
/// ```ignore
/// let amount: u64 = nondet_with(|x| *x > 0 && *x <= MAX_DEPOSIT);
/// ```
pub fn nondet_with<T: Nondet, F>(func: F) -> T
where
    F: FnOnce(&T) -> bool,
{
    Nondet::nondet_with(func)
}

/// Implements [`Nondet`] for a type by giving the expression that produces its
/// arbitrary value.
///
/// `nondet_impl! { Type, expr, "doc string" }` expands to
/// `impl Nondet for Type { fn nondet() -> Type { expr } }`, with the string
/// literal as the method's doc comment. `expr` is evaluated anew on each call
/// to `nondet()`.
///
/// The expansion names `Nondet` unqualified, so the trait must be in scope at
/// the invocation site (`use cvlr::nondet::Nondet;`). For structs and enums
/// whose fields all implement `Nondet`, `#[derive(Nondet)]` is usually
/// simpler; use this macro when the value needs a custom construction, e.g.
///
/// ```ignore
/// use cvlr::nondet::{nondet, nondet_impl, Nondet};
///
/// pub struct Wrapper(Pubkey);
/// nondet_impl! { Wrapper, Wrapper(Pubkey::new_from_array(nondet())), "Arbitrary pubkey" }
/// ```
///
/// The usual orphan rule applies: the type must be local to the invoking
/// crate.
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
