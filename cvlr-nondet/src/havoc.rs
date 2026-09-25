use {core::alloc::Layout, std::alloc::alloc};

mod rt_decls {
    extern "C" {
        pub fn memhavoc_c(data: *mut u8, sz: usize);
    }
}

#[cfg(feature = "rt")]
#[allow(dead_code)]
mod rt_imps {
    pub extern "C" fn memhavoc_c(data: *mut u8, sz: usize) {
        unsafe {
            data.write_bytes(0, sz);
        }
    }
}

/// Intended to overwrite `size` bytes starting at `data` with arbitrary values.
///
/// **The Certora Prover does not currently model this function**: the
/// underlying `memhavoc_c` builtin is treated as an unknown external call, so
/// the bytes keep whatever the Prover already knew about them. On memory that
/// has been written, this is a no-op; do not rely on it to forget a value. To
/// obtain unconstrained memory, allocate fresh memory with [`alloc_havoced`]
/// (fresh heap memory is unconstrained in the Prover regardless of this
/// call), or build the value with [`Nondet`](crate::Nondet).
///
/// # Safety
///
/// `data` must be valid for writes of `size` bytes. The bytes written may form
/// an invalid value for types with validity invariants (`bool`, `char`,
/// enums, references, `NonZero*`, ...), so only havoc memory that holds
/// plain-old-data (integers, byte arrays, and structs or arrays of them).
pub unsafe fn memhavoc(data: *mut u8, size: usize) {
    unsafe {
        rt_decls::memhavoc_c(data, size);
    }
}

/// Allocates a fresh `T` on the heap with arbitrary contents and returns a raw
/// pointer to it.
///
/// The memory is obtained from the global allocator and passed to
/// [`memhavoc`]; it is never freed. Every call returns a new allocation, so
/// the result does not alias any other object. The contents are arbitrary
/// because the Prover treats freshly allocated heap memory as unconstrained.
///
/// Useful for producing an arbitrary value of a foreign plain-old-data type
/// that has no [`Nondet`](crate::Nondet) implementation, e.g.
/// `*alloc_ref_havoced::<Stake>()`.
///
/// `T` must not be zero-sized, and must be a type for which every bit pattern
/// is a valid value (see [`memhavoc`]).
pub fn alloc_havoced<T: Sized>() -> *mut T {
    let layout = Layout::new::<T>();
    unsafe {
        let ptr = alloc(layout);
        memhavoc(ptr, layout.size());
        ptr as *mut T
    }
}

/// [`alloc_havoced`] as a shared reference.
///
/// Same requirements on `T` as [`alloc_havoced`]. Dereference and copy the
/// result (`*alloc_ref_havoced::<T>()`) to get an arbitrary `T` by value.
pub fn alloc_ref_havoced<T: Sized>() -> &'static T {
    unsafe { &*alloc_havoced::<T>() }
}

/// [`alloc_havoced`] as a mutable reference.
///
/// Same requirements on `T` as [`alloc_havoced`]. Use this when the code under
/// verification needs a `&mut T` to an arbitrary, unaliased object.
pub fn alloc_mut_ref_havoced<T: Sized>() -> &'static mut T {
    unsafe { &mut *alloc_havoced::<T>() }
}
