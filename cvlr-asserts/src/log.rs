mod rt_decls {
    extern "C" {
        #![allow(improper_ctypes)]
        // duplicated to avoid cvlr-assert depend on any other cvlr crate
        pub fn CVT_calltrace_attach_location(file: &str, line: u64);
    }
}

#[inline(always)]
pub fn add_loc(file: &str, line: u32) {
    unsafe {
        rt_decls::CVT_calltrace_attach_location(file, line as u64);
    }
}

#[cfg(not(feature = "no-loc"))]
#[macro_export]
macro_rules! cvlr_asserts_core_file {
    () => {
        ::core::file!()
    };
}

#[cfg(not(feature = "no-loc"))]
#[macro_export]
macro_rules! cvlr_asserts_core_line {
    () => {
        ::core::line!()
    };
}

#[cfg(feature = "no-loc")]
#[macro_export]
macro_rules! cvlr_asserts_core_file {
    () => {
        "<FILE>"
    };
}

#[cfg(feature = "no-loc")]
#[macro_export]
macro_rules! cvlr_asserts_core_line {
    () => {
        0u32
    };
}

#[macro_export]
macro_rules! add_loc {
    () => {
        $crate::log::add_loc(
            $crate::cvlr_asserts_core_file!(),
            $crate::cvlr_asserts_core_line!(),
        );
    };
}
