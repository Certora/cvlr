//! Reader for the `.cvlr_rules` section emitted by `#[rule]`.
//!
//! A record is a tag byte, a little-endian `u32` length, and that many payload
//! bytes. Tag 0 with payload `CV1` is the anchor. Tag 1 is one rule name.
//! Anything else, a short read, an empty name, or a name that is not UTF-8 is
//! corrupt. Duplicate tag-1 payloads are one name.

use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::path::Path;

const ELF_MAGIC: [u8; 4] = [0x7f, b'E', b'L', b'F'];
const ELFCLASS64: u8 = 2;
const ELFDATA2LSB: u8 = 1;
const PT_LOAD: u32 = 1;
const PF_X: u32 = 1;
const PF_W: u32 = 2;
const SHT_NOBITS: u32 = 8;
const SHF_ALLOC: u64 = 0x2;

const TAG_ANCHOR: u8 = 0;
const TAG_RULE: u8 = 1;
const ANCHOR: &[u8] = b"CV1";

#[derive(Debug)]
pub enum ReadError {
    Io(std::io::Error),
    /// Magic, class, or endian is not ELF64 little-endian.
    NotElf(&'static str),
    /// The file is ELF64 little-endian, and a later field is not usable.
    Malformed(&'static str),
    MissingSection,
    Corrupt {
        offset: usize,
    },
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReadError::Io(err) => write!(f, "{err}"),
            ReadError::NotElf(why) => write!(f, "not an ELF64 little-endian file: {why}"),
            ReadError::Malformed(why) => write!(f, "{why}"),
            ReadError::MissingSection => write!(f, "no .cvlr_rules section"),
            ReadError::Corrupt { offset } => {
                write!(f, ".cvlr_rules is corrupt at offset {offset}")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadKind {
    /// No `PT_LOAD` contains the section. A relocatable object has no loads.
    NotLoaded,
    Executable,
    ReadOnly,
    Writable,
}

#[derive(Debug)]
pub struct RulesInElf {
    pub names: BTreeSet<String>,
    pub anchor_count: usize,
    pub bytes: Vec<u8>,
    pub load: LoadKind,
    /// A `PT_LOAD` contains `.text`. The survival test uses this so "not in
    /// the text segment" is not true just because no text segment was found.
    pub saw_text_segment: bool,
    /// Some `PT_LOAD` contains both `.text` and `.cvlr_rules`.
    pub shares_text_segment: bool,
}

pub fn read_rules(path: &Path) -> Result<RulesInElf, ReadError> {
    let bytes = fs::read(path).map_err(ReadError::Io)?;
    let elf = parse_elf(&bytes)?;
    let section = elf
        .sections
        .iter()
        .find(|section| section.name == ".cvlr_rules")
        .ok_or(ReadError::MissingSection)?;
    if section.sh_type == SHT_NOBITS {
        return Err(ReadError::Malformed(".cvlr_rules is SHT_NOBITS"));
    }
    let data = slice_at(
        &bytes,
        section.offset,
        section.size,
        "section extends past the file",
    )?;
    let (names, anchor_count) = parse_records(data)?;
    let loads = elf
        .loads
        .iter()
        .filter(|load| contains(load, section))
        .collect::<Vec<_>>();
    let text_loads = elf
        .sections
        .iter()
        .filter(|candidate| candidate.name == ".text")
        .flat_map(|text| elf.loads.iter().filter(move |load| contains(load, text)))
        .collect::<Vec<_>>();
    let shares_text_segment = loads.iter().any(|load| {
        text_loads
            .iter()
            .any(|text_load| same_load(load, text_load))
    });
    Ok(RulesInElf {
        names,
        anchor_count,
        bytes: data.to_vec(),
        load: load_kind(&loads),
        saw_text_segment: !text_loads.is_empty(),
        shares_text_segment,
    })
}

fn same_load(left: &Load, right: &Load) -> bool {
    left.flags == right.flags && left.vaddr == right.vaddr && left.memsz == right.memsz
}

pub fn parse_records(data: &[u8]) -> Result<(BTreeSet<String>, usize), ReadError> {
    let mut names = BTreeSet::new();
    let mut anchor_count = 0;
    let mut offset = 0;
    while offset < data.len() {
        let start = offset;
        if data.len() - offset < 5 {
            return Err(ReadError::Corrupt { offset: start });
        }
        let tag = data[offset];
        let len = u32::from_le_bytes(data[offset + 1..offset + 5].try_into().unwrap()) as usize;
        offset += 5;
        if data.len() - offset < len {
            return Err(ReadError::Corrupt { offset: start });
        }
        let payload = &data[offset..offset + len];
        offset += len;
        match tag {
            TAG_ANCHOR => {
                if payload != ANCHOR {
                    return Err(ReadError::Corrupt { offset: start });
                }
                anchor_count += 1;
            }
            TAG_RULE => {
                if payload.is_empty() {
                    return Err(ReadError::Corrupt { offset: start });
                }
                let name = std::str::from_utf8(payload)
                    .map_err(|_| ReadError::Corrupt { offset: start })?;
                names.insert(name.to_string());
            }
            _ => return Err(ReadError::Corrupt { offset: start }),
        }
    }
    Ok((names, anchor_count))
}

pub fn rule_record(name: &str) -> Vec<u8> {
    let mut record = Vec::with_capacity(5 + name.len());
    record.push(TAG_RULE);
    record.extend_from_slice(&(name.len() as u32).to_le_bytes());
    record.extend_from_slice(name.as_bytes());
    record
}

struct ElfFile {
    sections: Vec<ElfSection>,
    loads: Vec<Load>,
}

struct ElfSection {
    name: String,
    sh_type: u32,
    flags: u64,
    addr: u64,
    offset: u64,
    size: u64,
}

struct Load {
    flags: u32,
    vaddr: u64,
    memsz: u64,
}

fn parse_elf(bytes: &[u8]) -> Result<ElfFile, ReadError> {
    if bytes.len() < 64 {
        return Err(ReadError::NotElf("shorter than an ELF header"));
    }
    if bytes[0..4] != ELF_MAGIC {
        return Err(ReadError::NotElf("bad magic"));
    }
    if bytes[4] != ELFCLASS64 {
        return Err(ReadError::NotElf("not ELF64"));
    }
    if bytes[5] != ELFDATA2LSB {
        return Err(ReadError::NotElf("not little-endian"));
    }
    let phoff = read_u64(bytes, 32)?;
    let shoff = read_u64(bytes, 40)?;
    let phentsize = read_u16(bytes, 54)? as usize;
    let phnum = read_u16(bytes, 56)? as usize;
    let shentsize = read_u16(bytes, 58)? as usize;
    let shnum = read_u16(bytes, 60)? as usize;
    let shstrndx = read_u16(bytes, 62)? as usize;
    if shentsize < 64 || shnum == 0 || shstrndx >= shnum {
        return Err(ReadError::Malformed("section headers do not parse"));
    }
    let shstr = section_header(bytes, shoff, shentsize, shstrndx)?;
    let shstrtab = slice_at(
        bytes,
        shstr.offset,
        shstr.size,
        "string table extends past the file",
    )?;
    let mut sections = Vec::with_capacity(shnum);
    for index in 0..shnum {
        let header = section_header(bytes, shoff, shentsize, index)?;
        let name = cstr_at(shstrtab, header.name_off as usize)?;
        sections.push(ElfSection {
            name,
            sh_type: header.sh_type,
            flags: header.flags,
            addr: header.addr,
            offset: header.offset,
            size: header.size,
        });
    }
    let mut loads = Vec::new();
    if phnum > 0 {
        if phentsize < 56 {
            return Err(ReadError::Malformed("program headers do not parse"));
        }
        for index in 0..phnum {
            let start = (phoff as usize)
                .checked_add(
                    index
                        .checked_mul(phentsize)
                        .ok_or(ReadError::Malformed("program header index overflow"))?,
                )
                .ok_or(ReadError::Malformed("program header offset overflow"))?;
            let header = bytes
                .get(start..start + 56)
                .ok_or(ReadError::Malformed("program header extends past the file"))?;
            let kind = u32::from_le_bytes(header[0..4].try_into().unwrap());
            if kind != PT_LOAD {
                continue;
            }
            loads.push(Load {
                flags: u32::from_le_bytes(header[4..8].try_into().unwrap()),
                vaddr: u64::from_le_bytes(header[16..24].try_into().unwrap()),
                memsz: u64::from_le_bytes(header[40..48].try_into().unwrap()),
            });
        }
    }
    Ok(ElfFile { sections, loads })
}

struct SectionHeader {
    name_off: u32,
    sh_type: u32,
    flags: u64,
    addr: u64,
    offset: u64,
    size: u64,
}

fn section_header(
    bytes: &[u8],
    shoff: u64,
    entsize: usize,
    index: usize,
) -> Result<SectionHeader, ReadError> {
    let start = (shoff as usize)
        .checked_add(
            index
                .checked_mul(entsize)
                .ok_or(ReadError::Malformed("section header index overflow"))?,
        )
        .ok_or(ReadError::Malformed("section header offset overflow"))?;
    let header = bytes
        .get(start..start + 64)
        .ok_or(ReadError::Malformed("section header extends past the file"))?;
    Ok(SectionHeader {
        name_off: u32::from_le_bytes(header[0..4].try_into().unwrap()),
        sh_type: u32::from_le_bytes(header[4..8].try_into().unwrap()),
        flags: u64::from_le_bytes(header[8..16].try_into().unwrap()),
        addr: u64::from_le_bytes(header[16..24].try_into().unwrap()),
        offset: u64::from_le_bytes(header[24..32].try_into().unwrap()),
        size: u64::from_le_bytes(header[32..40].try_into().unwrap()),
    })
}

fn contains(load: &Load, section: &ElfSection) -> bool {
    if section.flags & SHF_ALLOC == 0 || section.size == 0 {
        return false;
    }
    let Some(section_end) = section.addr.checked_add(section.size) else {
        return false;
    };
    let Some(load_end) = load.vaddr.checked_add(load.memsz) else {
        return false;
    };
    section.addr >= load.vaddr && section_end <= load_end
}

fn load_kind(loads: &[&Load]) -> LoadKind {
    if loads.is_empty() {
        return LoadKind::NotLoaded;
    }
    if loads.iter().any(|load| load.flags & PF_X != 0) {
        return LoadKind::Executable;
    }
    if loads.iter().any(|load| load.flags & PF_W != 0) {
        return LoadKind::Writable;
    }
    LoadKind::ReadOnly
}

fn read_u16(bytes: &[u8], offset: usize) -> Result<u16, ReadError> {
    let slice = bytes
        .get(offset..offset + 2)
        .ok_or(ReadError::Malformed("truncated header field"))?;
    Ok(u16::from_le_bytes(slice.try_into().unwrap()))
}

fn read_u64(bytes: &[u8], offset: usize) -> Result<u64, ReadError> {
    let slice = bytes
        .get(offset..offset + 8)
        .ok_or(ReadError::Malformed("truncated header field"))?;
    Ok(u64::from_le_bytes(slice.try_into().unwrap()))
}

fn slice_at<'a>(
    bytes: &'a [u8],
    offset: u64,
    size: u64,
    why: &'static str,
) -> Result<&'a [u8], ReadError> {
    let start = usize::try_from(offset).map_err(|_| ReadError::Malformed(why))?;
    let len = usize::try_from(size).map_err(|_| ReadError::Malformed(why))?;
    let end = start.checked_add(len).ok_or(ReadError::Malformed(why))?;
    bytes.get(start..end).ok_or(ReadError::Malformed(why))
}

fn cstr_at(bytes: &[u8], offset: usize) -> Result<String, ReadError> {
    let rest = bytes.get(offset..).ok_or(ReadError::Malformed(
        "section name is past the string table",
    ))?;
    let end = rest
        .iter()
        .position(|byte| *byte == 0)
        .ok_or(ReadError::Malformed("section name is not terminated"))?;
    let name = std::str::from_utf8(&rest[..end])
        .map_err(|_| ReadError::Malformed("section name is not UTF-8"))?;
    Ok(name.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(data: &[u8]) -> BTreeSet<String> {
        parse_records(data).unwrap().0
    }

    #[test]
    fn two_names_and_an_anchor() {
        let mut data = Vec::new();
        data.extend(rule_record("rule_beta"));
        data.extend([TAG_ANCHOR, 3, 0, 0, 0, b'C', b'V', b'1']);
        data.extend(rule_record("rule_alpha"));
        let (parsed, anchors) = parse_records(&data).unwrap();
        assert_eq!(
            parsed,
            BTreeSet::from(["rule_alpha".into(), "rule_beta".into()])
        );
        assert_eq!(anchors, 1);
    }

    #[test]
    fn duplicate_records_collapse() {
        let mut data = rule_record("rule_alpha");
        data.extend(rule_record("rule_alpha"));
        assert_eq!(names(&data), BTreeSet::from(["rule_alpha".into()]));
    }

    #[test]
    fn empty_section_has_no_names() {
        let (parsed, anchors) = parse_records(&[]).unwrap();
        assert!(parsed.is_empty());
        assert_eq!(anchors, 0);
    }

    #[test]
    fn truncated_header_is_corrupt() {
        let err = parse_records(&[1, 10, 0]).unwrap_err();
        assert!(matches!(err, ReadError::Corrupt { offset: 0 }));
    }

    #[test]
    fn truncated_payload_is_corrupt() {
        let err = parse_records(&[1, 2, 0, 0, 0, b'a']).unwrap_err();
        assert!(matches!(err, ReadError::Corrupt { offset: 0 }));
    }

    #[test]
    fn unknown_tag_is_corrupt() {
        let err = parse_records(&[2, 0, 0, 0, 0]).unwrap_err();
        assert!(matches!(err, ReadError::Corrupt { offset: 0 }));
    }

    #[test]
    fn bad_anchor_is_corrupt() {
        let err = parse_records(&[0, 3, 0, 0, 0, b'C', b'V', b'2']).unwrap_err();
        assert!(matches!(err, ReadError::Corrupt { offset: 0 }));
    }

    #[test]
    fn empty_name_is_corrupt() {
        let err = parse_records(&[1, 0, 0, 0, 0]).unwrap_err();
        assert!(matches!(err, ReadError::Corrupt { offset: 0 }));
    }

    #[test]
    fn bad_utf8_is_corrupt() {
        let err = parse_records(&[1, 1, 0, 0, 0, 0xff]).unwrap_err();
        assert!(matches!(err, ReadError::Corrupt { offset: 0 }));
    }

    #[test]
    fn short_file_is_not_elf_and_bad_section_headers_are_malformed() {
        let err = match parse_elf(&[0, 1, 2]) {
            Ok(_) => panic!("short file parsed as ELF"),
            Err(err) => err,
        };
        assert_eq!(
            err.to_string(),
            "not an ELF64 little-endian file: shorter than an ELF header"
        );

        let mut header = vec![0; 64];
        header[0..4].copy_from_slice(&[0x7f, b'E', b'L', b'F']);
        header[4] = ELFCLASS64;
        header[5] = ELFDATA2LSB;
        let err = match parse_elf(&header) {
            Ok(_) => panic!("header with no section table parsed as ELF"),
            Err(err) => err,
        };
        assert_eq!(err.to_string(), "section headers do not parse");
    }
}
