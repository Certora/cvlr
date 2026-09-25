mod rt_decls {
    #[cfg_attr(
        all(target_family = "wasm", target_os = "none"),
        link(wasm_import_module = "env")
    )]
    #[allow(improper_ctypes)]
    extern "C" {
        pub fn CVT_u128_leq(x: u128, y: u128) -> bool;
        pub fn CVT_u128_gt0(x: u128) -> bool;
        pub fn CVT_u128_ceil_div(x: u128, y: u128) -> u128;
    }
}

#[cfg(feature = "rt")]
#[allow(improper_ctypes_definitions)]
mod rt_impls {
    #[no_mangle]
    pub extern "C" fn CVT_u128_leq(x: u128, y: u128) -> bool {
        x <= y
    }

    #[no_mangle]
    pub extern "C" fn CVT_u128_gt0(x: u128) -> bool {
        x > 0
    }

    #[no_mangle]
    pub extern "C" fn CVT_u128_ceil_div(x: u128, y: u128) -> u128 {
        x.div_ceil(y)
    }
}

/// `x <= y` for `u128` operands.
///
/// # Prover support
///
/// This calls the `CVT_u128_leq` builtin, which was meant to let the Prover
/// reason about 128-bit arithmetic more cheaply than through the compiler's
/// multi-word lowering. Current Certora Prover releases no longer recognize
/// that builtin: the Solana front end now detects `u128` comparisons in the
/// compiled code and handles them specially on its own, and a call to
/// `CVT_u128_leq` is treated as an unknown external function whose result is
/// unconstrained. An assertion over the result can then fail spuriously, and an
/// assumption over it constrains nothing. Use the ordinary `u128` operator `x
/// <= y` instead.
pub fn cvlr_u128_leq(x: u128, y: u128) -> bool {
    unsafe { rt_decls::CVT_u128_leq(x, y) }
}

/// `x > 0` for a `u128` operand.
///
/// # Prover support
///
/// This calls the `CVT_u128_gt0` builtin, which was meant to let the Prover
/// reason about 128-bit arithmetic more cheaply than through the compiler's
/// multi-word lowering. Current Certora Prover releases no longer recognize
/// that builtin: the Solana front end now detects `u128` comparisons in the
/// compiled code and handles them specially on its own, and a call to
/// `CVT_u128_gt0` is treated as an unknown external function whose result is
/// unconstrained. An assertion over the result can then fail spuriously, and an
/// assumption over it constrains nothing. Use the ordinary `u128` operator `x >
/// 0` instead.
pub fn cvlr_u128_gt0(x: u128) -> bool {
    unsafe { rt_decls::CVT_u128_gt0(x) }
}

/// `x / y` rounded up, for `u128` operands.
///
/// With the `rt` feature this is `x.div_ceil(y)`, which panics if `y` is zero.
///
/// # Prover support
///
/// This calls the `CVT_u128_ceil_div` builtin, which was meant to let the
/// Prover reason about 128-bit arithmetic more cheaply than through the
/// compiler's multi-word lowering. Current Certora Prover releases no longer
/// recognize that builtin: a call to `CVT_u128_ceil_div` is treated as an
/// unknown external function whose result is unconstrained. An assertion over
/// the result can then fail spuriously, and an assumption over it constrains
/// nothing. Use the ordinary `u128` operator `x.div_ceil(y)` instead.
pub fn cvlr_u128_ceil_div(x: u128, y: u128) -> u128 {
    unsafe { rt_decls::CVT_u128_ceil_div(x, y) }
}
