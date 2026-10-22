//! `#[rule]` writes `.cvlr_rules`, and `cargo certora-sbf` and a Wasm build keep
//! the section.

mod section;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Deserialize;

use cvlr_rule_manifest::rule_record;
use section::{LoadKind, ReadError};

#[test]
fn rule_record_is_in_the_host_object() {
    let staged = Stage::new("host");
    let cvlr = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    fs::create_dir_all(staged.path.join("src")).unwrap();
    fs::write(
        staged.path.join("Cargo.toml"),
        format!(
            r#"[package]
name = "rule_manifest_host"
version = "0.0.0"
edition = "2021"
publish = false

[workspace]

[dependencies]
cvlr = {{ path = {cvlr} }}
"#,
            cvlr = toml_basic_string(&cvlr.display().to_string()),
        ),
    )
    .unwrap();
    fs::write(
        staged.path.join("src/lib.rs"),
        "use cvlr::prelude::*;\n\n#[rule]\nfn rule_alpha() {}\n",
    )
    .unwrap();

    let target = workspace_target().join("rule-manifest-host");
    let output = cargo(
        &staged.path,
        &[
            "rustc",
            "--manifest-path",
            staged.path.join("Cargo.toml").to_str().unwrap(),
            "--message-format=json",
            "--color",
            "never",
            "--",
            "--emit=obj",
            "-C",
            "codegen-units=1",
        ],
        &target,
    );
    assert!(
        output.status.success(),
        "host rustc --emit=obj failed:\n{}",
        tail(&output.stderr),
    );
    let objects = object_files(
        &String::from_utf8_lossy(&output.stdout),
        "rule_manifest_host",
    );
    let mut hits = Vec::new();
    let mut saw_elf = false;
    let mut saw_non_elf = false;
    for object in &objects {
        match section::read_rules(object) {
            Ok(rules) => {
                saw_elf = true;
                hits.push((object.clone(), rules));
            }
            Err(ReadError::MissingSection) => saw_elf = true,
            Err(ReadError::NotElf(_)) => saw_non_elf = true,
            Err(err) => panic!("reading {}: {err}", object.display()),
        }
    }
    if hits.is_empty() && !saw_elf && saw_non_elf {
        eprintln!("skipping rule_record_is_in_the_host_object: host rustc emitted no ELF object");
        return;
    }
    assert_eq!(
        hits.len(),
        1,
        "expected one object with .cvlr_rules, found {hits:?} among {objects:?}"
    );
    let (_, rules) = &hits[0];
    assert_eq!(
        rules.bytes,
        rule_record("rule_alpha"),
        "object section bytes are not the tag-1 record for rule_alpha"
    );
    assert_eq!(
        rules.names.iter().map(String::as_str).collect::<Vec<_>>(),
        ["rule_alpha"]
    );
    assert_eq!(rules.anchor_count, 0);
}

#[test]
fn rule_manifest_survives_certora_sbf() {
    if !certora_sbf_on_path() || !platform_tools_v143_installed() {
        eprintln!(
            "skipping rule_manifest_survives_certora_sbf: cargo-certora-sbf is not on PATH or platform tools v1.43 are not installed"
        );
        return;
    }

    let staged = stage_fixture("sbf");
    let manifest_path = staged.path.join("Cargo.toml");

    // Inside the package so certora-sbf can report `executables` relative to
    // `project_directory`. A target dir outside that prefix makes it fail.
    let target = staged.path.join("target");
    let output = cargo(
        &staged.path,
        &[
            "certora-sbf",
            "--json",
            "--no-rustup",
            "--no-tools-install",
            "--tools-version",
            "v1.43",
            "--manifest-path",
            manifest_path.to_str().unwrap(),
            "--features",
            "certora",
        ],
        &target,
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "cargo certora-sbf failed:\n{}\n{}",
        tail(&output.stderr),
        stdout,
    );
    let summary = certora_summary(&stdout);
    let artifact = {
        let path = PathBuf::from(&summary.executables);
        if path.is_absolute() {
            path
        } else {
            PathBuf::from(summary.project_directory).join(path)
        }
    };
    let rules = section::read_rules(&artifact)
        .unwrap_or_else(|err| panic!("reading {}: {err}", artifact.display()));
    assert_eq!(
        rules.names.iter().map(String::as_str).collect::<Vec<_>>(),
        ["rule_alpha", "rule_beta"],
    );
    assert!(!rules.names.contains("rule_cfg_removed"));
    assert!(!rules.names.contains("not_a_rule"));
    assert!(rules.anchor_count >= 1, "tag-0 CV1 anchor is absent");
    assert_eq!(
        rules.load,
        LoadKind::ReadOnly,
        ".cvlr_rules is not in a non-executable PT_LOAD: {rules:?}"
    );
    assert!(
        rules.saw_text_segment,
        ".text is not in a PT_LOAD, so the segment check would be vacuous: {rules:?}"
    );
    assert!(
        !rules.shares_text_segment,
        ".cvlr_rules shares the .text PT_LOAD: {rules:?}"
    );
}

#[test]
fn rule_manifest_survives_wasm() {
    if !wasm_target_installed() {
        eprintln!("skipping rule_manifest_survives_wasm: {WASM_TARGET} is not installed");
        return;
    }

    let staged = stage_fixture("wasm");
    let target = staged.path.join("target");
    let output = cargo(
        &staged.path,
        &[
            "build",
            "--release",
            "--target",
            WASM_TARGET,
            "--manifest-path",
            staged.path.join("Cargo.toml").to_str().unwrap(),
            // The Prover supplies the intrinsics. Without it, `rt` stubs them.
            "--features",
            "certora,cvlr/rt",
        ],
        &target,
    );
    assert!(
        output.status.success(),
        "wasm build failed:\n{}",
        tail(&output.stderr),
    );
    let artifact = target
        .join(WASM_TARGET)
        .join("release/rule_manifest_fixture.wasm");
    let data = section::read_wasm_section(&artifact)
        .unwrap_or_else(|err| panic!("reading {}: {err}", artifact.display()));
    let (names, anchor_count) = section::parse_records(&data)
        .unwrap_or_else(|err| panic!("reading {}: {err}", artifact.display()));
    assert_eq!(
        names.iter().map(String::as_str).collect::<Vec<_>>(),
        ["rule_alpha", "rule_beta"],
    );
    assert_eq!(anchor_count, 1, "tag-0 CV1 anchor is absent or repeated");
}

const WASM_TARGET: &str = "wasm32-unknown-unknown";

fn wasm_target_installed() -> bool {
    let Ok(output) = Command::new("rustc")
        .args(["--print", "sysroot"])
        .stdin(Stdio::null())
        .output()
    else {
        return false;
    };
    let sysroot = String::from_utf8_lossy(&output.stdout);
    Path::new(sysroot.trim())
        .join("lib/rustlib")
        .join(WASM_TARGET)
        .is_dir()
}

/// The fixture, copied to a scratch directory, depending on this `cvlr`.
fn stage_fixture(label: &str) -> Stage {
    let staged = Stage::new(label);
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/rule_manifest");
    copy_tree(&fixture, &staged.path);
    let manifest_path = staged.path.join("Cargo.toml");
    let manifest = fs::read_to_string(&manifest_path).unwrap();
    let cvlr = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_path = "\"../../../cvlr\"";
    assert!(
        manifest.contains(fixture_path),
        "fixture manifest has no cvlr path to rewrite"
    );
    fs::write(
        &manifest_path,
        manifest.replace(
            fixture_path,
            &toml_basic_string(&cvlr.display().to_string()),
        ),
    )
    .unwrap();
    staged
}

fn certora_sbf_on_path() -> bool {
    // The binary only accepts `--version` when Cargo invokes it as
    // `cargo certora-sbf`. Cargo also finds it under `~/.cargo/bin`.
    Command::new("cargo")
        .args(["certora-sbf", "--version"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}

fn platform_tools_v143_installed() -> bool {
    let Some(root) = std::env::var_os("CERTORA_PLATFORM_TOOLS_ROOT")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".cache/solana")))
    else {
        return false;
    };
    ["platform-tools-certora", "platform-tools"]
        .into_iter()
        .any(|flavour| {
            root.join("v1.43")
                .join(flavour)
                .join("rust/bin/rustc")
                .is_file()
        })
}

fn workspace_target() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target")
}

fn cargo(dir: &Path, args: &[&str], target_dir: &Path) -> Output {
    Command::new("cargo")
        .args(args)
        .current_dir(dir)
        .env("CARGO_TARGET_DIR", target_dir)
        .env("CARGO_TERM_COLOR", "never")
        .env("CARGO_INCREMENTAL", "0")
        .output()
        .expect("spawn cargo")
}

fn object_files(stdout: &str, package: &str) -> Vec<PathBuf> {
    let mut objects = Vec::new();
    for line in stdout.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let message: CargoMessage = serde_json::from_str(line)
            .unwrap_or_else(|err| panic!("cargo JSON line does not match: {err}\n{line}"));
        let CargoMessage::CompilerArtifact { target, filenames } = message else {
            continue;
        };
        if target.name != package {
            continue;
        }
        for path in filenames {
            if path.extension().is_some_and(|ext| ext == "o") {
                objects.push(path);
                continue;
            }
            // `cargo rustc -- --emit=obj` writes the object next to the
            // hashed rmeta, but the JSON filenames list the rmeta and rlib.
            if let Some(object) = object_beside_rmeta(&path) {
                objects.push(object);
            }
        }
    }
    objects
}

#[derive(Deserialize)]
#[serde(tag = "reason")]
enum CargoMessage {
    #[serde(rename = "compiler-artifact")]
    CompilerArtifact {
        target: CargoTarget,
        filenames: Vec<PathBuf>,
    },
    #[serde(other)]
    Other,
}

#[derive(Deserialize)]
struct CargoTarget {
    name: String,
}

fn object_beside_rmeta(path: &Path) -> Option<PathBuf> {
    let name = path.file_name()?.to_str()?;
    let stem = name.strip_prefix("lib")?.strip_suffix(".rmeta")?;
    let object = path.with_file_name(format!("{stem}.o"));
    object.is_file().then_some(object)
}

fn certora_summary(stdout: &str) -> CertoraSummary {
    serde_json::from_str(stdout)
        .unwrap_or_else(|err| panic!("certora-sbf JSON summary does not match: {err}\n{stdout}"))
}

#[derive(Deserialize)]
struct CertoraSummary {
    project_directory: String,
    executables: String,
}

fn toml_basic_string(value: &str) -> String {
    let mut out = String::from("\"");
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{0008}' => out.push_str("\\b"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\u{000c}' => out.push_str("\\f"),
            '\r' => out.push_str("\\r"),
            ch if ch.is_control() => out.push_str(&format!("\\u{:04X}", u32::from(ch))),
            ch => out.push(ch),
        }
    }
    out.push('"');
    out
}

fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).unwrap();
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name();
        if name == "target" || name == "Cargo.lock" {
            continue;
        }
        let to = dst.join(&name);
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &to);
        } else {
            fs::copy(entry.path(), &to).unwrap();
        }
    }
}

fn tail(bytes: &[u8]) -> String {
    let text = String::from_utf8_lossy(bytes);
    let mut start = text.len().saturating_sub(8_000);
    while start > 0 && !text.is_char_boundary(start) {
        start -= 1;
    }
    text[start..].to_string()
}

#[test]
fn certora_summary_reads_pretty_json() {
    let stdout = r#"{
  "project_directory": "/work/pkg",
  "executables": "target/deploy/rule_manifest.so",
  "sources": [],
  "success": true,
  "return_code": 0
}"#;
    let summary = certora_summary(stdout);
    assert_eq!(summary.project_directory, "/work/pkg");
    assert_eq!(summary.executables, "target/deploy/rule_manifest.so");
}

#[test]
fn object_files_uses_the_target_name() {
    let stdout = r#"{"reason":"compiler-artifact","target":{"name":"other"},"filenames":["/tmp/rule_manifest_host/other.o"]}
{"reason":"compiler-artifact","target":{"name":"rule_manifest_host"},"filenames":["/tmp/out/rule_manifest_host.o"]}
{"reason":"build-finished","success":true}
"#;
    let objects = object_files(stdout, "rule_manifest_host");
    assert_eq!(
        objects,
        vec![PathBuf::from("/tmp/out/rule_manifest_host.o")]
    );
}

#[test]
#[should_panic(expected = "filenames")]
fn object_files_rejects_a_compiler_artifact_without_filenames() {
    let stdout = r#"{"reason":"compiler-artifact","target":{"name":"rule_manifest_host"}}"#;
    object_files(stdout, "rule_manifest_host");
}

#[test]
fn toml_basic_string_escapes_quotes_and_backslashes() {
    assert_eq!(toml_basic_string("a\"b\\c"), r#""a\"b\\c""#);
    assert_eq!(toml_basic_string("a\nb"), r#""a\nb""#);
}

#[test]
fn tail_cuts_on_a_char_boundary() {
    let text = format!("{}é{}", "b".repeat(10), "a".repeat(7_999));
    let got = tail(text.as_bytes());
    assert_eq!(got, &text[10..]);
    assert!(got.starts_with('é'));
}

struct Stage {
    path: PathBuf,
}

impl Stage {
    fn new(label: &str) -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "cvlr-rule-manifest-{label}-{}-{nanos}",
            std::process::id()
        ));
        fs::create_dir_all(&path).unwrap();
        Self { path }
    }
}

impl Drop for Stage {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}
