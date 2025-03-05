use {core::alloc::Layout, std::alloc::alloc};

mod rt_decls {
    extern "C" {
        pub fn memhavoc_c(data: *mut u8, sz: usize);
    }
}

#[cfg(feature = "rt")]
mod rt_imps {
    pub extern "C" fn memhavoc_c(data: *mut u8, sz: usize) {
        unsafe {
            data.write_bytes(0, sz);
        }
    }
}

pub fn memhavoc(data: *mut u8, size: usize) {
    unsafe {
        rt_decls::memhavoc_c(data, size);
    }
}

pub fn alloc_havoced<T: Sized>() -> *mut T {
    let layout = Layout::new::<T>();
    unsafe {
        let ptr = alloc(layout);
        memhavoc(ptr, layout.size());
        ptr as *mut T
    }
}

pub fn alloc_ref_havoced<T: Sized>() -> &'static T {
    unsafe { &*alloc_havoced::<T>() }
}

pub fn alloc_mut_ref_havoced<T: Sized>() -> &'static mut T {
    unsafe { &mut *alloc_havoced::<T>() }
}
