#![no_std]

pub mod log;
mod core;
pub mod asserts;


pub use core::*;

#[deprecated = "Replaced by cvlr"]
pub mod cvt {
    pub use crate::cvlr_assert as cvt_assert;
    pub use crate::cvlr_assume as cvt_assume;
    pub use crate::cvlr_satisfy as cvt_satisfy;
    pub use crate::cvlr_vacuity_check as cvt_vacuity_check;
}

