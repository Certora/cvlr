#![deprecated = "Replaced by cvlr::clog!"]

/// Deprecated: use `clog!("message")`.
///
/// Intended to log the source text of `tag` as a message line.
#[macro_export]
macro_rules! cvt_cex_print_tag {
    ($tag: expr) => {
        $crate::log(stringify($tag))
    };
}

/// Deprecated: use `clog!`.
///
/// `cvt_cex_print_u64!(tag, v1, v2, ...)` logs the values, each cast with
/// `as u64`, under the source text of `tag`, as consecutive entries of up to
/// three values each.
#[macro_export]
macro_rules! cvt_cex_print_u64 {
    ($tag: expr $(,)?) => {};
    ($tag: expr, $x: expr) => { $crate::log_u64(stringify!($tag), $x as u64) };
    ($tag: expr, $x: expr, $y: expr) => { unsafe {$crate::CVT_calltrace_print_u64_2(stringify!($tag), $x as u64,$y as u64)}};
    ($tag: expr, $v: expr, $x: expr, $y: expr $(,$z:expr)*) => {
        unsafe {
            $crate::CVT_calltrace_print_u64_3(
                stringify!($tag),
                $v as u64,
                $x as u64,
                $y as u64
            )
        };
        cvt_cex_print_u64!($tag $(,$z)*)
    };
}

/// Deprecated: use `clog!`.
///
/// `cvt_cex_print_i64!(tag, v1, v2, ...)` logs the values, each cast with
/// `as i64`, under the source text of `tag`, as consecutive entries of up to
/// three values each.
#[macro_export]
macro_rules! cvt_cex_print_i64 {
    ($tag: expr $(,)?) => {};
    ($tag: expr, $x: expr) => { $crate::log_i64(stringify!($tag), $x as i64)};
    ($tag: expr, $x: expr, $y: expr) => { unsafe {$crate::CVT_calltrace_print_i64_2(stringify!($tag), $x as i64,$y as i64)}};
    ($tag: expr, $v: expr, $x: expr, $y: expr $(,$z:expr)*) =>
    {
	unsafe {$crate::CVT_calltrace_print_i64_3(stringify!($tag), $v as i64, $x as i64,$y as i64)};
	cvt_cex_print_i64!($tag $(,$z)*)
    };
}
