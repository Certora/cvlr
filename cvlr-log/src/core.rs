pub mod rt_decls {
    #[allow(improper_ctypes)]
    extern "C" {
        pub fn CVT_calltrace_print_tag(tag: &str);

        pub fn CVT_calltrace_print_u64_1(tag: &str, x: u64);
        pub fn CVT_calltrace_print_u64_2(tag: &str, x: u64, y: u64);
        pub fn CVT_calltrace_print_u64_3(tag: &str, x: u64, y: u64, z: u64);
        pub fn CVT_calltrace_print_u128(tag: &str, x: u128);

        pub fn CVT_calltrace_print_i64_1(tag: &str, x: i64);
        pub fn CVT_calltrace_print_i64_2(tag: &str, x: i64, y: i64);
        pub fn CVT_calltrace_print_i64_3(tag: &str, x: i64, y: i64, z: i64);
        pub fn CVT_calltrace_print_i128(tag: &str, x: i128);

        pub fn CVT_calltrace_print_string(tag: &str, v: &str);

        pub fn CVT_calltrace_print_u64_as_fixed(tag: &str, x: u64, y: u64);

        pub fn CVT_calltrace_print_location(file: &str, line: u64);
        pub fn CVT_calltrace_attach_location(file: &str, line: u64);

        pub fn CVT_calltrace_scope_start(name: &str);
        pub fn CVT_calltrace_scope_end(name: &str);
    }
}

#[cfg(feature = "rt")]
mod rt_impls {
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_tag(_tag: &str) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_u64_1(_tag: &str, _x: u64) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_u64_2(_tag: &str, _x: u64, _y: u64) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_u64_3(_tag: &str, _x: u64, _y: u64, _z: u64) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_u128(_tag: &str, _x: u128) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_i64_1(_tag: &str, _x: i64) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_i64_2(_tag: &str, _x: i64, _y: i64) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_i64_3(_tag: &str, _x: i64, _y: i64, _z: i64) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_i128(_tag: &str, _x: i128) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_u64_as_fixed(_tag: &str, _x: u64, _y: u64) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_string(_tag: &str, v: &str) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_location(_tag: &str, v: u64) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_attach_location(_tag: &str, v: u64) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_scope_start(name: &str) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_scope_end(name: &str) {}
}
pub use rt_decls::*;

#[derive(Default)]
pub struct CvlrLogger;

impl CvlrLogger {
    #[inline(always)]
    pub fn new() -> Self {
        Self {}
    }
    #[inline(always)]
    pub fn log(&mut self, v: &str) {
        unsafe {
            CVT_calltrace_print_tag(v);
        }
    }

    #[inline(always)]
    pub fn log_str(&mut self, t: &str, v: &str) {
        unsafe {
            CVT_calltrace_print_string(t, v);
        }
    }

    #[inline(always)]
    pub fn log_u64(&mut self, t: &str, v: u64) {
        unsafe {
            CVT_calltrace_print_u64_1(t, v);
        }
    }

    #[inline(always)]
    pub fn log_u64_2(&mut self, t: &str, v0: u64, v1: u64) {
        unsafe {
            CVT_calltrace_print_u64_2(t, v0, v1);
        }
    }

    #[inline(always)]
    pub fn log_u64_3(&mut self, t: &str, v0: u64, v1: u64, v2: u64) {
        unsafe {
            CVT_calltrace_print_u64_3(t, v0, v1, v2);
        }
    }

    #[inline(always)]
    pub fn log_u128(&mut self, t: &str, v: u128) {
        unsafe {
            CVT_calltrace_print_u128(t, v);
        }
    }

    #[inline(always)]
    pub fn log_i64(&mut self, t: &str, v: i64) {
        unsafe {
            CVT_calltrace_print_i64_1(t, v);
        }
    }

    #[inline(always)]
    pub fn log_i128(&mut self, t: &str, v: i128) {
        unsafe {
            CVT_calltrace_print_i128(t, v);
        }
    }

    #[inline(always)]
    pub fn log_u64_as_fp(&mut self, t: &str, v: u64, b: u64) {
        unsafe {
            CVT_calltrace_print_u64_as_fixed(t, v, b);
        }
    }

    #[inline(always)]
    pub fn log_loc(&mut self, file: &str, line: u32) {
        unsafe {
            CVT_calltrace_print_location(file, line as u64);
        }
    }

    #[inline(always)]
    pub fn add_loc(&mut self, file: &str, line: u32) {
        unsafe {
            CVT_calltrace_attach_location(file, line as u64);
        }
    }

    #[inline(always)]
    pub fn log_scope_start(&mut self, scope: &str) {
        unsafe {
            CVT_calltrace_scope_start(scope);
        }
    }

    #[inline(always)]
    pub fn log_scope_end(&mut self, scope: &str) {
        unsafe {
            CVT_calltrace_scope_end(scope);
        }
    }
}

#[inline(always)]
pub fn log(v: &str) {
    let mut logger = CvlrLogger::new();
    logger.log(v);
}

#[inline(always)]
pub fn log_u64_as_fp(t: &str, v: u64, b: u64) {
    let mut logger = CvlrLogger::new();
    logger.log_u64_as_fp(t, v, b);
}

#[inline(always)]
pub fn log_scope_start(scope: &str) {
    let mut logger = CvlrLogger::new();
    logger.log_scope_start(scope);
}

#[inline(always)]
pub fn log_scope_end(scope: &str) {
    let mut logger = CvlrLogger::new();
    logger.log_scope_end(scope);
}

macro_rules! expose_log_fn {
    ($name: ident, $ty: ty) => {
        #[inline(always)]
        pub fn $name(t: &str, v: $ty) {
            let mut logger = CvlrLogger::new();
            logger.$name(t, v)
        }
    };
}

expose_log_fn! {log_str, &str}
expose_log_fn! {log_u64, u64}
expose_log_fn! {log_i64, i64}
expose_log_fn! {log_u128, u128}
expose_log_fn! {log_i128, i128}
expose_log_fn! {log_loc, u32}
expose_log_fn! {add_loc, u32}
