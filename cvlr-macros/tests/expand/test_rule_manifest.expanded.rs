use cvlr_macros::rule;
#[no_mangle]
fn rule_alpha() {
    #[allow(non_upper_case_globals)]
    #[used]
    #[export_name = "cvlr_rule_manifest.rule_alpha"]
    #[link_section = ".cvlr_rules"]
    static CVLR_RULE_MANIFEST_rule_alpha: [u8; 15] = [
        1, 10, 0, 0, 0, b'r', b'u', b'l', b'e', b'_', b'a', b'l', b'p', b'h', b'a',
    ];
    { ::cvlr_log::log_rule_location("<FILE>", 0u32 as u64) };
    ::cvlr_asserts::cvlr_sanity_checked(true);
}
#[no_mangle]
fn r#type() {
    #[allow(non_upper_case_globals)]
    #[used]
    #[export_name = "cvlr_rule_manifest.type"]
    #[link_section = ".cvlr_rules"]
    static CVLR_RULE_MANIFEST_type: [u8; 9] = [1, 4, 0, 0, 0, b't', b'y', b'p', b'e'];
    { ::cvlr_log::log_rule_location("<FILE>", 0u32 as u64) };
    ::cvlr_asserts::cvlr_sanity_checked(true);
}
fn main() {}
