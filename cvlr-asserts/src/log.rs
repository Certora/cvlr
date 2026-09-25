mod rt_decls {
    #[cfg_attr(
        all(target_family = "wasm", target_os = "none"),
        link(wasm_import_module = "env")
    )]
    extern "C" {
        #![allow(improper_ctypes)]
        // duplicated to avoid cvlr-assert depend on any other cvlr crate
        pub fn CVT_calltrace_attach_location(file: &str, line: u64);
    }
}

/// Attaches the source location `file:line` to the next assertion, satisfy,
/// log entry or scope in the call trace.
///
/// Emits no entry of its own: the Prover removes the call and attaches the
/// location to the next operation that can carry one, which becomes the
/// target of that entry's jump-to-source link and, for an assertion, its
/// reported location. [`cvlr_assert!`](crate::cvlr_assert) and
/// [`cvlr_satisfy!`](crate::cvlr_satisfy) call it for you through
/// [`add_loc!`](crate::add_loc). `file` must be a string literal and `line` a
/// constant.
#[inline(always)]
pub fn add_loc(file: &str, line: u32) {
    unsafe {
        rt_decls::CVT_calltrace_attach_location(file, line as u64);
    }
}

/// Expands to the current source file name, or `"<FILE>"` with feature
/// `no-loc`.
///
/// Implementation detail of [`add_loc!`](crate::add_loc); exported only so the
/// macro can reach it from other crates. The `no-loc` feature makes macro
/// expansions independent of source locations (for snapshot tests).
#[cfg(not(feature = "no-loc"))]
#[macro_export]
macro_rules! cvlr_asserts_core_file {
    () => {
        ::core::file!()
    };
}

/// Expands to the current source line, or `0u32` with feature `no-loc`.
///
/// Implementation detail of [`add_loc!`](crate::add_loc); exported only so the
/// macro can reach it from other crates. The `no-loc` feature makes macro
/// expansions independent of source locations (for snapshot tests).
#[cfg(not(feature = "no-loc"))]
#[macro_export]
macro_rules! cvlr_asserts_core_line {
    () => {
        ::core::line!()
    };
}

/// Expands to the current source file name, or `"<FILE>"` with feature
/// `no-loc`.
///
/// Implementation detail of [`add_loc!`](crate::add_loc); exported only so the
/// macro can reach it from other crates. The `no-loc` feature makes macro
/// expansions independent of source locations (for snapshot tests).
#[cfg(feature = "no-loc")]
#[macro_export]
macro_rules! cvlr_asserts_core_file {
    () => {
        "<FILE>"
    };
}

/// Expands to the current source line, or `0u32` with feature `no-loc`.
///
/// Implementation detail of [`add_loc!`](crate::add_loc); exported only so the
/// macro can reach it from other crates. The `no-loc` feature makes macro
/// expansions independent of source locations (for snapshot tests).
#[cfg(feature = "no-loc")]
#[macro_export]
macro_rules! cvlr_asserts_core_line {
    () => {
        0u32
    };
}

/// Attaches the location of the macro call to the next assertion, satisfy,
/// log entry or scope; see [`add_loc`](fn@crate::log::add_loc).
///
/// Already done by [`cvlr_assert!`](crate::cvlr_assert) and
/// [`cvlr_satisfy!`](crate::cvlr_satisfy). Call it directly before a
/// [`cvlr_assert_checked`](crate::cvlr_assert_checked) or similar low-level
/// call that should link back to its source line.
#[macro_export]
macro_rules! add_loc {
    () => {
        $crate::log::add_loc(
            $crate::cvlr_asserts_core_file!(),
            $crate::cvlr_asserts_core_line!(),
        );
    };
}
