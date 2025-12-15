#![no_std]

pub mod u128_arith;

pub mod asserts {
    pub use cvlr_asserts::*;
}

pub mod mathint {
    pub use cvlr_mathint::*;
}

pub mod nondet {
    pub use cvlr_nondet::*;
}

pub mod log {
    pub use cvlr_log::*;
}

pub mod macros {
    pub use cvlr_macros::*;
}

pub mod derive {
    pub use cvlr_derive::*;
}

pub mod prelude {
    pub use super::asserts::*;

    pub use super::log::cvlr_log as clog;
    pub use super::nondet::nondet;
    pub use super::nondet::nondet as cvlr_nondet;

    pub use cvlr_early_panic::early_panic as cvlr_early_panic;
    pub use cvlr_hook::cvlr_hook_on_entry;
    pub use cvlr_hook::cvlr_hook_on_exit;
    pub use cvlr_macros::rule as cvlr_rule;

    pub use cvlr_early_panic::early_panic;
    pub use cvlr_hook::cvlr_hook_on_entry as hook_on_entry;
    pub use cvlr_hook::cvlr_hook_on_exit as hook_on_exit;
    pub use cvlr_macros::mock_fn;
    pub use cvlr_macros::rule;

    pub use cvlr_macros::{
        cvlr_assert_all, cvlr_assert_that, cvlr_assume_all, cvlr_assume_that, cvlr_eval_all,
        cvlr_eval_that,
    };

    pub use cvlr_derive::{CvlrLog, Nondet};
}

pub use prelude::*;

pub use crate::mathint::{is_u128, is_u16, is_u32, is_u64, is_u8};
