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

pub mod spec {
    pub use cvlr_spec::*;
}

pub mod fixed { 
    pub use cvlr_fixed::*;
}

pub mod decimal { 
    pub use cvlr_decimal::*;
}

pub mod prelude {
    pub use super::asserts::*;

    pub use super::log::cvlr_log as clog;
    pub use super::nondet::nondet;
    pub use super::nondet::nondet as cvlr_nondet;

    pub use cvlr_early_panic::early_panic as cvlr_early_panic;
    pub use cvlr_hook::cvlr_hook_on_entry;
    pub use cvlr_hook::cvlr_hook_on_exit;
    pub use __macro_support::rule as cvlr_rule;

    pub use cvlr_early_panic::early_panic;
    pub use cvlr_hook::cvlr_hook_on_entry as hook_on_entry;
    pub use cvlr_hook::cvlr_hook_on_exit as hook_on_exit;
    pub use __macro_support::mock_fn;
    pub use __macro_support::rule;

    pub use super::macros::{
        cvlr_assert_all, cvlr_assert_that, cvlr_assume_all, cvlr_assume_that, cvlr_eval_all,
        cvlr_eval_that,
    };

    pub use super::derive::{CvlrLog, Nondet};

    pub use super::spec::*;
}

pub use prelude::*;

pub use crate::mathint::{is_u128, is_u16, is_u32, is_u64, is_u8};
