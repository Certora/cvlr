mod rt_decls {
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

pub fn cvlr_u128_leq(x: u128, y: u128) -> bool {
    unsafe { rt_decls::CVT_u128_leq(x, y) }
}

pub fn cvlr_u128_gt0(x: u128) -> bool {
    unsafe { rt_decls::CVT_u128_gt0(x) }
}

pub fn cvlr_u128_ceil_div(x: u128, y: u128) -> u128 {
    unsafe { rt_decls::CVT_u128_ceil_div(x, y) }
}
