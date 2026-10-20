#![no_std]
//! The `.cvlr_rules` section, which lists the rules a program compiled.
//!
//! A record is a tag byte, a little-endian `u32` length, and that many payload
//! bytes. Tag [`TAG_ANCHOR`] carries [`ANCHOR_PAYLOAD`]. Tag [`TAG_RULE`]
//! carries one rule name, UTF-8 and non-empty. The linker concatenates the
//! records of every object, in no particular order, and a name may repeat.
//!
//! The anchor is linked into any program that links this crate, including one
//! that compiles no rule, so a missing section is distinct from a section that
//! lists no rules. Each rule-defining macro emits its record with
//! [`rule_static`] (feature `tokens`).
//!
//! Apple targets omit the section: Mach-O rejects a bare ELF section name.

#[cfg(feature = "tokens")]
extern crate alloc;

/// The section name, in ELF objects and as a Wasm custom section.
pub const SECTION: &str = ".cvlr_rules";
/// Bytes before a record's payload: the tag and the length.
pub const HEADER_LEN: usize = 5;
/// The tag of the anchor record.
pub const TAG_ANCHOR: u8 = 0;
/// The tag of a rule-name record.
pub const TAG_RULE: u8 = 1;
/// The payload of the anchor record.
pub const ANCHOR_PAYLOAD: [u8; 3] = *b"CV1";
/// The complete anchor record.
pub const ANCHOR_RECORD: [u8; HEADER_LEN + ANCHOR_PAYLOAD.len()] = {
    let len = (ANCHOR_PAYLOAD.len() as u32).to_le_bytes();
    [
        TAG_ANCHOR,
        len[0],
        len[1],
        len[2],
        len[3],
        ANCHOR_PAYLOAD[0],
        ANCHOR_PAYLOAD[1],
        ANCHOR_PAYLOAD[2],
    ]
};
/// The prefix of a rule record's exported symbol; the rule name follows.
///
/// A rule name cannot contain `.`, so no rule export collides with
/// [`ANCHOR_EXPORT`], and two rules with one name fail to link.
pub const RULE_EXPORT_PREFIX: &str = "cvlr_rule_manifest.";
/// The exported symbol of the anchor record.
pub const ANCHOR_EXPORT: &str = "cvlr_rule_manifest_anchor";

// Attribute values must be literals; these repeat `ANCHOR_EXPORT` and `SECTION`.
#[used]
#[export_name = "cvlr_rule_manifest_anchor"]
#[cfg_attr(not(target_vendor = "apple"), link_section = ".cvlr_rules")]
static ANCHOR: [u8; ANCHOR_RECORD.len()] = ANCHOR_RECORD;

/// The tag-1 record for `name`.
///
/// # Panics
///
/// If `name` is longer than `u32::MAX` bytes.
#[cfg(feature = "tokens")]
pub fn rule_record(name: &str) -> alloc::vec::Vec<u8> {
    let len = u32::try_from(name.len()).expect("a rule name fits in a u32");
    let mut record = alloc::vec::Vec::with_capacity(HEADER_LEN + name.len());
    record.push(TAG_RULE);
    record.extend_from_slice(&len.to_le_bytes());
    record.extend_from_slice(name.as_bytes());
    record
}

/// A static holding the record for the rule `fn_name`, in [`SECTION`].
///
/// The static is valid as an item or as a statement in the rule's body. It is
/// never read, so it does not run. The rule name is `fn_name` without a raw
/// `r#` prefix, which is the name a `#[no_mangle]` function exports.
#[cfg(feature = "tokens")]
pub fn rule_static(fn_name: &syn::Ident) -> syn::ItemStatic {
    use alloc::format;
    use alloc::string::ToString;
    use alloc::vec::Vec;
    use proc_macro2::Literal;
    use syn::ext::IdentExt;

    let rule_name = fn_name.unraw().to_string();
    let static_ident =
        quote::format_ident!("CVLR_RULE_MANIFEST_{}", rule_name, span = fn_name.span());
    let export_name = format!("{RULE_EXPORT_PREFIX}{rule_name}");
    let record = rule_record(&rule_name);
    let record_len = Literal::usize_unsuffixed(record.len());
    let bytes: Vec<Literal> = record
        .into_iter()
        .enumerate()
        .map(|(index, byte)| {
            if index >= HEADER_LEN && byte.is_ascii_graphic() && byte != b'\'' && byte != b'\\' {
                Literal::byte_character(byte)
            } else {
                Literal::u8_unsuffixed(byte)
            }
        })
        .collect();
    syn::parse_quote! {
        #[allow(non_upper_case_globals)]
        #[used]
        #[export_name = #export_name]
        #[cfg_attr(not(target_vendor = "apple"), link_section = #SECTION)]
        static #static_ident: [u8; #record_len] = [#(#bytes),*];
    }
}

#[cfg(all(test, feature = "tokens"))]
mod tests {
    use super::*;

    #[test]
    fn rule_record_is_tag_length_name() {
        assert_eq!(rule_record("ab"), [TAG_RULE, 2, 0, 0, 0, b'a', b'b']);
    }

    #[test]
    fn anchor_record_is_tag_length_payload() {
        assert_eq!(ANCHOR_RECORD, [TAG_ANCHOR, 3, 0, 0, 0, b'C', b'V', b'1']);
    }
}
