//! Fixture for `rule_manifest_survives_certora_sbf`.
//!
//! Two rules are compiled. `rule_cfg_removed` is not. `not_a_rule` is
//! exported and is not a rule. `entrypoint` calls a rule, which links `cvlr`
//! and therefore the anchor record.

use cvlr::prelude::*;

#[rule]
pub fn rule_alpha() {}

#[rule]
pub fn rule_beta() {}

#[cfg(any())]
#[rule]
pub fn rule_cfg_removed() {}

#[no_mangle]
pub fn not_a_rule() {}

#[no_mangle]
pub fn entrypoint() {
    rule_alpha();
}
