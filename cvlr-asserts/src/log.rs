mod rt_decls {
    extern "C" {
        #![allow(improper_ctypes)]
        // duplicated to avoid cvlr-assert depend on any other cvlr crate
        pub fn CVT_calltrace_attach_location(file: &str, line: u64);
    }
}

#[cfg(feature = "rt")]
mod rt_impls {
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_attach_location(_tag: &str, v: u64) {}
}

#[inline(always)]
pub fn add_loc(file: &str, line: u32) {
    unsafe {
        rt_decls::CVT_calltrace_attach_location(file, line as u64);
    }
}

#[macro_export]
macro_rules! add_loc {
    () => {
        $crate::log::add_loc(core::file!(), core::line!());
    };
}
